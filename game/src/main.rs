use nova::graphics;
use nova::graphics::pipeline;
use nova::graphics::rendering;
use nova::graphics::window;
use nova::graphics::{Mesh, Vertex};
use nova::math::algebra::*;
use nova::utils::Droppable;
use std::iter;

/// Main entry point of the program.
pub fn main() -> Result<(), String> {
  let sink = bflog::LogSink::new(
    std::io::stdout(),
    bflog::Format::Modern,
    bflog::LevelFilter::Trace,
  );

  let mut log = bflog::Logger::new(&sink).with_src("game");
  let mut gfx = graphics::Context::new("nova-game", 1)
    .map_err(|err| format!("Could not create graphics context: {}", err))?;

  log.trace("Created graphics context.");

  let shaders = pipeline::PipelineShaderSet::load_defaults(&gfx.device);

  let mut renderer = rendering::Renderer::new(&gfx.queues.graphics);

  let descriptor_set_layout = pipeline::DescriptorSetLayout::new()
    .texture()
    .create(&gfx.device);

  let pipeline = pipeline::Pipeline::new()
    .render_pass(renderer.pass())
    .shaders(shaders)
    .vertex_buffer::<graphics::Vertex>()
    .push_constant::<graphics::Color>()
    .push_constant::<Matrix4<f32>>()
    .descriptor_set_layout(&descriptor_set_layout)
    .create();

  log.trace("Created pipeline.");

  let command_pool = rendering::CommandPool::new(&gfx.queues.graphics);

  log.trace("Created renderer arnd command pool.");

  let quad = Mesh::new(
    &gfx.device,
    &[
      Vertex::new([-0.5, -0.5], [1.0, 1.0, 1.0, 1.0], [1.0, 0.0]),
      Vertex::new([0.5, -0.5], [1.0, 1.0, 1.0, 1.0], [0.0, 0.0]),
      Vertex::new([0.5, 0.5], [1.0, 1.0, 1.0, 1.0], [0.0, 1.0]),
      Vertex::new([-0.5, 0.5], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0]),
    ],
    &[0, 1, 2, 2, 3, 0],
  );

  let mut texture_loader = rendering::TextureLoader::new(&gfx.queues.transfer);

  let texture = texture_loader.load(
    &image::load_from_memory(include_bytes!("../assets/do-it.jpg"))
      .expect("could not load texture")
      .to_rgba(),
  );

  let sampler = rendering::TextureSampler::new(&gfx.device);

  log.trace("Created mesh and texture/sampler pair.");

  let descriptor_pool = pipeline::DescriptorPool::new(&descriptor_set_layout, 1);

  let descriptor_set = pipeline::DescriptorSet::new(
    &descriptor_pool,
    &[pipeline::Descriptor::SampledTexture(&texture, &sampler)],
  );

  log.trace("Created descriptor set.");

  let mut swapchain = Droppable::<window::Swapchain>::dropped();

  loop {
    gfx.window.update();

    if gfx.window.is_closed() {
      break;
    }

    if !swapchain.is_dropped() && swapchain.size() != gfx.window.size() {
      swapchain.drop();
    }

    let (framebuffer, framebuffer_semaphore) = loop {
      if swapchain.is_dropped() {
        let size = gfx.window.size();

        swapchain =
          window::Swapchain::new(renderer.pass(), gfx.window.raw_surface_mut(), size).into();

        let actual_size = swapchain.size();

        log
          .info("Created swapchain.")
          .with("width", &actual_size.x)
          .with("height", &actual_size.y);
      }

      match swapchain.acquire_framebuffer() {
        Ok(fb) => break fb,
        Err(_) => swapchain.drop(),
      };
    };

    let mut cmd =
      rendering::CommandBuffer::new(&command_pool, rendering::CommandBufferKind::Secondary);

    cmd.begin_with(renderer.pass(), &framebuffer);

    cmd.bind_pipeline(&pipeline);

    cmd.push_constant(0, &graphics::Color([1.0, 1.0, 1.0, 1.0]));
    cmd.push_constant(1, &Matrix4::<f32>::identity());

    cmd.bind_descriptor_set(0, &descriptor_set);
    cmd.bind_vertex_buffer(0, quad.vertex_buffer());
    cmd.bind_index_buffer(quad.index_buffer());
    cmd.draw_indexed(quad.indices());

    cmd.finish();

    let render_semaphore = renderer.render(&framebuffer, &framebuffer_semaphore, iter::once(cmd));

    let result = gfx.queues.graphics.present(
      iter::once((swapchain.as_ref(), framebuffer.index())),
      iter::once(render_semaphore),
    );

    if let Err(_) = result {
      swapchain.drop();
    }
  }

  Ok(())
}
