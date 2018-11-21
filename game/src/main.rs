// TODO: Remove when RLS supports it.
extern crate nova;

use nova::ecs;
use nova::graphics;
use nova::log;
use nova::time;
use nova::utils::Ring;
use nova::window::{self, Window};
use std::process::exit;
use std::sync::Arc;

pub fn main() {
  let mut engine = nova::Engine::new();

  let log = log::get_logger(&mut engine).with_source("game");

  let (window, mut event_source) = Window::create().unwrap_or_else(|err| {
    log.error(format_args!("Could not create window: {}", err));
    exit(1)
  });

  let device = graphics::Device::create().unwrap_or_else(|err| {
    log
      .error(format_args!("Could not create device: {}", err))
      .with("backend", graphics::backend::NAME);

    exit(1)
  });

  let graphics_queue = graphics::device::get_graphics_queue(&device);

  let mut presenter = graphics::present::Presenter::new(&device, &window);

  let render_pass = Arc::new(graphics::render::RenderPass::new(&device));
  let mut renderer = graphics::render::Renderer::new(&render_pass);

  let command_pool = Arc::new(graphics::commands::CommandPool::new(
    &device,
    graphics_queue.family_id(),
  ));

  let mut submissions = Ring::new(2, |_| {
    graphics::device::FencedSubmission::new(&graphics_queue)
  });

  let mut semaphores = Ring::new(2, |_| Arc::new(graphics::sync::Semaphore::new(&device)));

  ecs::put_resource(&mut engine, window);

  let mut frame_limiter = time::FrameLimiter::new(&engine, 60.0);

  while !window::is_closing(&mut engine) {
    let backbuffer = presenter.begin(); // May block for vsync.

    frame_limiter.begin_frame();

    let submission = submissions.advance();

    submission.clear(); // May block if a previous frame is executing.

    renderer.attach(&backbuffer);

    event_source.poll();
    window::process_events(&mut engine, event_source.events());

    // TODO: Run game logic.

    ecs::maintain(&mut engine);

    let mut cmd =
      graphics::commands::Commands::new(&command_pool, graphics::commands::CommandLevel::Primary);

    cmd.begin();
    renderer.begin(&mut cmd);

    // TODO: Draw things.

    renderer.finish(&mut cmd);
    cmd.finish();

    submission.add_commands(cmd);
    submission.wait_for_output(&backbuffer);
    submission.signal_finished(semaphores.advance());
    submission.submit();

    backbuffer.present(submission.signal_semaphores());

    frame_limiter.end_frame(); // May block if frame was shorter than vsync.
  }

  // Wait for the device to be idle before exiting.
  device.wait_idle();
}
