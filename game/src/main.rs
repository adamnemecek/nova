// TODO: Remove when RLS supports it.
extern crate nova;

use nova::ecs;
use nova::graphics;
use nova::log;
use nova::time;
use nova::window::Window;
use std::process::exit;
use std::sync::Arc;
use std::thread;

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

  let mut submission = graphics::device::Submission::new();
  let mut fence = graphics::sync::Fence::new(&device);
  let semaphore = Arc::new(graphics::sync::Semaphore::new(&device));

  ecs::put_resource(&mut engine, window);
  ecs::put_resource(&mut engine, time::Clock::new());

  let mut frame_timer = time::FrameTimer::new(&engine);

  frame_timer.long_delta_time = Some(time::Duration::ONE_60TH_SEC * 1.1);
  frame_timer.long_frame_time = Some(time::Duration::ONE_144TH_SEC);

  loop {
    // Acquire a backbuffer from the presenter. This is first because it may
    // block until a vertical refresh.
    let backbuffer = presenter.begin();

    renderer.attach(&backbuffer);

    let clock: &mut time::Clock = ecs::get_resource_mut(&mut engine);

    frame_timer.begin_frame();

    clock.elapse(frame_timer.delta_time());

    let window: &mut Window = ecs::get_resource_mut(&mut engine);

    event_source.poll();

    window.process_events(event_source.events());

    if window.is_closing() {
      break;
    }

    // Run game logic.

    ecs::maintain(&mut engine);

    frame_timer.end_frame();

    let mut cmd =
      graphics::commands::Commands::new(&command_pool, graphics::commands::CommandLevel::Primary);

    cmd.begin();
    renderer.begin(&mut cmd);

    // Draw things.

    renderer.finish(&mut cmd);
    cmd.finish();

    fence.wait_and_reset();
    submission.clear();

    submission.add_commands(cmd);

    submission.wait_on(
      backbuffer.semaphore(),
      graphics::render::pipeline::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
    );

    submission.signal(&semaphore);

    graphics_queue.lock().submit(&submission, Some(&fence));

    backbuffer.present(&submission.signal_semaphores);

    if frame_timer.frame_time() < time::Duration::ONE_MILLI {
      thread::sleep(std::time::Duration::from_millis(1));
    }
  }

  device.wait_idle();
}
