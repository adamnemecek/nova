mod fps;
mod graphics;
//mod panels;

use self::graphics::image;
use self::graphics::pipeline;
use self::graphics::{Mesh, Vertex, Window};
use nova::ecs;
use nova::math::Matrix4;
use std::iter;

/// Main entry point of the program.
pub fn main() -> Result<(), String> {
  let sink = bflog::LogSink::new(
    std::io::stdout(),
    bflog::Format::Modern,
    bflog::LevelFilter::Trace,
  );

  let mut log = bflog::Logger::new(&sink).with_src("game");

  let backend = graphics::backend::Instance::create("nova-game", 1).into();

  log
    .trace("Created backend.")
    .with("name", &graphics::backend::NAME);

  let mut window =
    Window::new(&backend).map_err(|err| format!("Could not create window: {}", err))?;

  log
    .trace("Created window.")
    .with("width", &window.size().width())
    .with("height", &window.size().height());

  let mut gpu = graphics::device::Gpu::new(&backend)
    .map_err(|err| format!("Could not create graphics device: {}", err))?;

  log.trace("Created graphics device.");

  let mut renderer = graphics::Renderer::new(&gpu.queues.graphics, &window, &log);

  log.trace("Created renderer.");

  let command_pool = graphics::CommandPool::new(&gpu.queues.graphics);

  log.trace("Created command pool.");

  let pipeline = graphics::pipeline::create_default(renderer.pass());

  log.trace("Created graphics pipeline.");

  let quad = Mesh::new(
    &gpu.device,
    &[
      Vertex::new([-0.5, -0.5], [1.0, 1.0, 1.0, 1.0], [1.0, 0.0]),
      Vertex::new([0.5, -0.5], [1.0, 1.0, 1.0, 1.0], [0.0, 0.0]),
      Vertex::new([0.5, 0.5], [1.0, 1.0, 1.0, 1.0], [0.0, 1.0]),
      Vertex::new([-0.5, 0.5], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0]),
    ],
    &[0, 1, 2, 2, 3, 0],
  );

  log.trace("Created quad mesh.");

  let mut image_loader = image::Loader::new(&gpu.queues.transfer);

  let image = image_loader.load(
    &mut gpu.queues.transfer,
    &image::Source::from_bytes(include_bytes!("../assets/do-it.jpg"))
      .map_err(|err| format!("Could not load image data: {}", err))?,
  );

  let sampler = image::Sampler::new(&gpu.device);

  log.trace("Created quad texture.");

  let descriptor_pool = pipeline::DescriptorPool::new(pipeline.descriptor_set_layout().unwrap(), 1);

  let descriptor_set = pipeline::DescriptorSet::new(
    &descriptor_pool,
    &[pipeline::Descriptor::Texture(&image, &sampler)],
  );

  log.trace("Created quad texture descriptor set.");

  let mut ecs = ecs::Context::new();

  ecs::put_resource(&mut ecs, gpu.device.clone());

  let mut dispatcher = ecs::Dispatcher::new()
    .system("fps::Counter", &[], fps::Counter::new())
    .setup(&mut ecs);

  loop {
    window.update();

    if window.is_closing() {
      break;
    }

    let framebuffer = renderer.begin_frame(&mut window);

    dispatcher.dispatch(&mut ecs);

    let mut cmd = graphics::Commands::new(&command_pool, graphics::commands::Level::Primary);

    cmd.begin();

    cmd.begin_render_pass(renderer.pass(), &framebuffer);

    cmd.bind_pipeline(&pipeline);

    cmd.push_constant(0, &graphics::Color([1.0, 1.0, 1.0, 1.0]));
    cmd.push_constant(1, &Matrix4::<f32>::identity());

    cmd.bind_descriptor_set(0, &descriptor_set);
    cmd.bind_vertex_buffer(0, quad.vertex_buffer());
    cmd.bind_index_buffer(quad.index_buffer());
    cmd.draw_indexed(quad.index_count());

    cmd.finish();

    renderer.submit_frame(&mut gpu.queues.graphics, iter::once(cmd));

    let fps = ecs::get_resource_mut::<fps::Stats>(&mut ecs);

    if fps.total_secs > 3.0 {
      log
        .trace("Running.")
        .with("fps", &fps.fps)
        .with("avg_ms", &fps.avg_ms);

      fps.total_secs = 0.0;
    }

    std::thread::yield_now();
  }

  Ok(())
}
