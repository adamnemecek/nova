use nova::*;

//mod clock;
mod graphics;
//mod ui;
//mod panels;

use self::graphics::image;
use self::graphics::pipeline;
use self::graphics::window::Window;
use self::graphics::{Mesh, Vertex};
use self::math::Matrix4;
use std::iter;
use std::sync::Arc;
use std::time;

/// Main entry point of the program.
pub fn main() -> Result<(), String> {
  let sink = bflog::LogSink::new(
    std::io::stdout(),
    bflog::Format::Modern,
    bflog::LevelFilter::Trace,
  );

  bflog::Logger::new(&sink).make_global().unwrap();

  let mut log = bflog::Logger::new(&sink).with_src("game");

  let backend = graphics::backend::Instance::create("nova-game", 1).into();

  log
    .trace("Created backend.")
    .with("name", &graphics::backend::NAME);

  let (mut window, mut event_source) =
    Window::new(&backend).map_err(|err| format!("Could not create window: {}", err))?;

  log
    .trace("Created window.")
    .with("width", &window.size().width())
    .with("height", &window.size().height());

  let mut gpu = graphics::device::Gpu::new(&backend, window.surface())
    .map_err(|err| format!("Could not create graphics device: {}", err))?;

  log
    .trace("Created graphics device.")
    .with("name", &gpu.device.name());

  let mut renderer = graphics::Renderer::new(&gpu.queues.graphics, &mut window, &log);

  log.trace("Created renderer.");

  let command_pool = graphics::CommandPool::new(&gpu.queues.graphics);

  log.trace("Created command pool.");

  let pipeline = graphics::pipeline::create_default(renderer.render_pass());

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

  let mut image_loader =
    image::Loader::new(gpu.queues.transfer.as_ref().unwrap_or(&gpu.queues.graphics));

  let image = image_loader.load(
    gpu
      .queues
      .transfer
      .as_mut()
      .unwrap_or(&mut gpu.queues.graphics),
    &image::Source::from_bytes(include_bytes!("../assets/do-it.jpg"))
      .map_err(|err| format!("Could not load image data: {}", err))?,
  );

  let sampler = image::Sampler::new(&gpu.device);

  log.trace("Created quad texture.");

  let mut descriptor_pool = pipeline::DescriptorPool::new(&pipeline.descriptor_layouts()[0], 1);

  let descriptor_set = descriptor_pool.allocate_set(vec![pipeline::Descriptor::Texture(
    Arc::new(image),
    Arc::new(sampler),
  )]);

  log.trace("Created quad texture descriptor set.");

  let ctx = &mut ecs::Context::new();
  let mut dispatcher = ecs::Dispatcher::new().setup(ctx);

  ecs::put_resource(ctx, window);

  loop {
    let start_time = time::Instant::now();
    let window: &mut Window = ecs::get_resource_mut(ctx);

    event_source.update();

    window.update(event_source.events());

    if window.is_closing() {
      break;
    }

    let framebuffer = renderer.begin_frame(window);

    dispatcher.dispatch(ctx);

    ctx.update();

    let mut cmd = graphics::Commands::new(&command_pool, graphics::commands::Level::Primary);

    cmd.begin();

    cmd.begin_render_pass(renderer.render_pass(), &framebuffer);

    cmd.bind_pipeline(&pipeline);

    cmd.push_constant(0, &graphics::Color4::new(1.0, 1.0, 1.0, 1.0));
    cmd.push_constant(1, &Matrix4::<f32>::identity());

    cmd.bind_descriptor_set(0, &descriptor_set);
    cmd.bind_vertex_buffer(0, quad.vertex_buffer());
    cmd.bind_index_buffer(quad.index_buffer());
    cmd.draw_indexed(quad.index_count());

    cmd.finish_render_pass();

    cmd.finish();

    renderer.submit_frame(&mut gpu.queues.graphics, iter::once(cmd));

    let duration = time::Instant::now() - start_time;

    if duration > time::Duration::from_millis(20) {
      log.warn("Long frame.").with(
        "duration",
        &(duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9),
      );
    } else if duration < time::Duration::from_millis(1) {
      // log.warn("Short frame.").with(
      //   "duration",
      //   &(duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9),
      // );

      std::thread::yield_now();
    }
  }

  gpu.device.wait_idle();

  Ok(())
}
