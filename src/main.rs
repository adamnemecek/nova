use nova::graphics;
use nova::log::{self, Drain as _};
use nova::math::Size;
use nova::time;
use nova::window;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
  let logger = log::terminal_compact();

  log::set_global_logger(&logger);

  let graphics = graphics::Instance::new(&logger)?;

  return Ok(());

  let window = window::open(window::Options {
    size: Size::new(2560.0, 1440.0),
    resizable: false,
    ..Default::default()
  });

  time::loop_at_frequency(60.0, |loop_ctx| {
    while let Some(event) = window.next_event() {
      if let window::Event::CloseRequested = event {
        loop_ctx.stop();
      }
    }
  });

  Ok(())
}
