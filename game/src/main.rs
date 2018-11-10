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

  let (gfx_device, gfx_queues) =
    graphics::Device::open::<graphics::device::DefaultQueueSet>(&backend)
      .map_err(|err| format!("Could not create graphics device: {}", err))?;

  log.trace("Opened graphics device.");

  let mut renderer = graphics::Renderer::new(&gfx_queues.graphics, &window, &log);

  log.trace("Created renderer.");

  let command_pool = graphics::CommandPool::new(&gfx_queues.graphics);

  log.trace("Created command pool.");

  let pipeline = graphics::pipeline::create_default(renderer.pass());

  log.trace("Created graphics pipeline.");

  let quad = Mesh::new(
    &gfx_device,
    &[
      Vertex::new([-0.5, -0.5], [1.0, 1.0, 1.0, 1.0], [1.0, 0.0]),
      Vertex::new([0.5, -0.5], [1.0, 1.0, 1.0, 1.0], [0.0, 0.0]),
      Vertex::new([0.5, 0.5], [1.0, 1.0, 1.0, 1.0], [0.0, 1.0]),
      Vertex::new([-0.5, 0.5], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0]),
    ],
    &[0, 1, 2, 2, 3, 0],
  );

  log.trace("Created quad mesh.");

  let mut image_loader = image::Loader::new(&gfx_queues.transfer);

  let image = image_loader.load(
    &image::Source::from_bytes(include_bytes!("../assets/do-it.jpg"))
      .map_err(|err| format!("Could not load image data: {}", err))?,
  );

  let sampler = image::Sampler::new(&gfx_device);

  log.trace("Created quad texture.");

  let descriptor_pool = pipeline::DescriptorPool::new(pipeline.descriptor_set_layout().unwrap(), 1);

  let descriptor_set = pipeline::DescriptorSet::new(
    &descriptor_pool,
    &[pipeline::Descriptor::Texture(&image, &sampler)],
  );

  log.trace("Created quad texture descriptor set.");

  loop {
    window.update();

    if window.is_closed() {
      break;
    }

    renderer.resize(window.size());

    let framebuffer = renderer.begin_frame();

    let mut cmd = graphics::CommandBuffer::new(&command_pool, graphics::CommandBufferKind::Primary);

    cmd.begin();

    cmd.begin_pass(renderer.pass(), &framebuffer);

    cmd.bind_pipeline(&pipeline);

    cmd.push_constant(0, &graphics::Color([1.0, 1.0, 1.0, 1.0]));
    cmd.push_constant(1, &Matrix4::<f32>::identity());

    cmd.bind_descriptor_set(0, &descriptor_set);
    cmd.bind_vertex_buffer(0, quad.vertex_buffer());
    cmd.bind_index_buffer(quad.index_buffer());
    cmd.draw_indexed(quad.index_count());

    cmd.finish();

    renderer.submit_frame(iter::once(cmd));
  }

  Ok(())
}
