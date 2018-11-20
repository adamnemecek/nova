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

  let mut frame_timer = time::FrameTimer::new();

  loop {
    let clock: &mut time::Clock = ecs::get_resource_mut(&mut engine);

    frame_timer.begin_frame();

    clock.elapse(frame_timer.delta_time());

    log
      .trace("Frame began.")
      .with("delta_time", frame_timer.delta_time())
      .with("average", frame_timer.avg_delta_time())
      .with("fps", frame_timer.avg_fps());

    let window: &mut Window = ecs::get_resource_mut(&mut engine);

    event_source.poll();

    window.process_events(event_source.events());

    if window.is_closing() {
      break;
    }

    let backbuffer = presenter.begin();

    renderer.attach(&backbuffer);

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

    log
      .trace("Frame ended.")
      .with("frame_time", frame_timer.frame_time())
      .with("average", frame_timer.avg_frame_time());

    if frame_timer.frame_time() < time::Duration::ONE_MILLI {
      thread::sleep(std::time::Duration::from_millis(1));
    }
  }
}
