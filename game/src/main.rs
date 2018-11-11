mod graphics;

use self::graphics::image;
use self::graphics::pipeline;
use self::graphics::{Mesh, Vertex, Window};
use nova::math::algebra::*;
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
    .with("width", &window.size().x)
    .with("height", &window.size().y);

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

  use std::time;

  let mut duration = time::Duration::default();
  let mut frames = 0;

  loop {
    std::thread::yield_now();

    let start = time::Instant::now();

    window.update();

    if window.is_closed() {
      break;
    }

    renderer.resize(window.size());

    let framebuffer = renderer.begin_frame();

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

    // Track frame rate and duration.
    duration += time::Instant::now() - start;
    frames += 1;

    const FPS_WINDOW: time::Duration = time::Duration::from_secs(3);

    if duration > FPS_WINDOW {
      let secs = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
      let fps = frames as f64 / secs;

      duration = time::Duration::default();
      frames = 0;

      log
        .trace("Running…")
        .with("fps", &fps)
        .with("avg_ms", &(fps.recip() * 1000.0));
    }
  }

  Ok(())
}
