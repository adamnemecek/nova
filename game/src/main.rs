// TODO: Remove when RLS supports it.
extern crate bflog;
extern crate nova;

use nova::*;

//mod clock;
mod graphics;
//mod ui;
//mod panels;

use self::graphics::{Mesh, Vertex};
use nova::graphics::backend;
use nova::graphics::commands;
use nova::graphics::image;
use nova::graphics::render;
use nova::graphics::render::descriptor::{Descriptor, DescriptorPool};
use nova::graphics::sync;
use nova::graphics::window::{self, Window};
use nova::graphics::Color4;
use nova::math::Matrix4;
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

  let backend = backend::Instance::create("nova-game", 1).into();

  log.trace("Created backend.").with("name", &backend::NAME);

  let (mut window, mut event_source) =
    Window::create(&backend).map_err(|err| format!("Could not create window: {}", err))?;

  log
    .trace("Created window.")
    .with("width", &window.size().width())
    .with("height", &window.size().height());

  let mut gpu = nova::graphics::Context::create(&backend, window.surface())
    .map_err(|err| format!("Could not create graphics device: {}", err))?;

  log
    .trace("Created graphics device.")
    .with("name", &gpu.device.name());

  let mut presenter = window::Presenter::new(&gpu.device, &mut window);

  let render_pass = Arc::new(render::RenderPass::new(&gpu.device));
  let mut renderer = render::Renderer::new(&render_pass);

  log.trace("Created renderer.");

  let mut submission = commands::Submission::new();

  let command_pool = Arc::new(commands::CommandPool::new(
    &gpu.device,
    gpu.queues.graphics.family_id(),
  ));

  log.trace("Created command pool.");

  let pipeline = graphics::pipeline::create_default(renderer.pass())
    .map_err(|err| format!("Could not create graphics pipeline: {}", err))?;

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

  let mut image_loader = image::Loader::new(
    &gpu.device,
    gpu
      .queues
      .transfer
      .as_ref()
      .unwrap_or(&gpu.queues.graphics)
      .family_id(),
  );

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

  let mut descriptor_pool = DescriptorPool::new(&pipeline.descriptor_layouts()[0], 1);

  let descriptor_set = descriptor_pool.allocate_set(vec![Descriptor::Texture(
    Arc::new(image),
    Arc::new(sampler),
  )]);

  log.trace("Created quad texture descriptor set.");

  let ctx = &mut ecs::Context::new();
  let mut dispatcher = ecs::DispatcherBuilder::new().build(ctx);

  ecs::put_resource(ctx, window);

  let mut fence = sync::Fence::new(&gpu.device);
  let semaphore = Arc::new(sync::Semaphore::new(&gpu.device));

  loop {
    let start_time = time::Instant::now();
    let window: &mut Window = ecs::get_resource_mut(ctx);

    event_source.update();

    window.update(event_source.events());

    if window.is_closing() {
      break;
    }

    dispatcher.dispatch(ctx);

    ctx.update();

    let backbuffer = presenter.begin();

    renderer.attach(&backbuffer);

    let mut cmd = commands::Commands::new(&command_pool, commands::CommandLevel::Primary);

    cmd.begin();

    renderer.begin(&mut cmd);

    cmd.bind_pipeline(&pipeline);

    cmd.push_constant(0, &Color4::new(1.0, 1.0, 1.0, 1.0));
    cmd.push_constant(1, &Matrix4::<f32>::identity());

    cmd.bind_descriptor_set(0, &descriptor_set);
    cmd.bind_vertex_buffer(0, quad.vertex_buffer());
    cmd.bind_index_buffer(quad.index_buffer());
    cmd.draw_indexed(quad.index_count());

    renderer.finish(&mut cmd);

    cmd.finish();

    fence.wait_and_reset();

    submission.clear();

    submission.add_commands(cmd);

    submission.wait_on(
      backbuffer.semaphore(),
      render::pipeline::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
    );

    submission.signal(&semaphore);

    gpu.queues.graphics.submit(&submission, Some(&fence));

    backbuffer.present(&mut gpu.queues.graphics, &submission.signal_semaphores);

    let duration = time::Instant::now() - start_time;

    if duration > time::Duration::from_millis(20) {
      log.warn("Long frame.").with(
        "duration",
        &(duration.as_secs() as f64 + f64::from(duration.subsec_nanos()) * 1e-9),
      );
    } else if duration < time::Duration::from_millis(1) {
      log.warn("Short frame.").with(
        "duration",
        &(duration.as_secs() as f64 + f64::from(duration.subsec_nanos()) * 1e-9),
      );

      std::thread::yield_now();
    }
  }

  gpu.device.wait_idle();

  Ok(())
}
