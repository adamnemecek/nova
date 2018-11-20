#![feature(duration_float)]

// TODO: Remove when RLS supports it.
extern crate nova;

use nova::ecs;
use nova::log;
use nova::time;
use nova::window::Window;
use std::process::exit;
use std::thread;

pub fn main() {
  let mut engine = nova::Engine::new();

  let log = log::get_logger(&mut engine).with_source("game");

  let (window, mut event_source) = Window::create().unwrap_or_else(|err| {
    log.error(format_args!("Could not create window: {}", err));
    exit(1)
  });

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

    ecs::maintain(&mut engine);

    frame_timer.end_frame();

    log
      .trace("Frame ended.")
      .with("frame_time", frame_timer.frame_time())
      .with("average", frame_timer.avg_frame_time());

    if frame_timer.frame_time() < time::Duration::ONE_MILLI {
      thread::sleep(std::time::Duration::from_millis(1));
    }
  }
}
