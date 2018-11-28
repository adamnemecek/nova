// TODO: Remove when RLS supports it.
extern crate nova;

use nova::log;
use nova::time;
use nova::window::Window;

pub fn main() {
  let mut engine = nova::Engine::new();
  let log = log::get_logger(&mut engine).with_source("game");

  time::setup(&mut engine);

  let mut rate_limiter = time::RateLimiter::new();
  let mut window = Window::create(&mut engine).expect("Could not create window");

  loop {
    rate_limiter.begin();

    time::update_clock(&mut engine);

    log.trace("Frame.").with(
      "delta_time",
      engine.get_resource_mut::<time::Clock>().delta_time,
    );

    window.update(&mut engine);

    rate_limiter.wait_for_full_duration(time::Duration::ONE_60TH_SEC);
  }
}
