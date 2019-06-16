use nova::graphics;
use nova::log;
use nova::time;
use nova::window;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
  let logger = log::terminal_compact();

  log::set_global_logger(&logger);

  let window = window::open(window::Options {
    size: (2560.0, 1440.0).into(),
    ..Default::default()
  });

  let _graphics = graphics::Context::new(&logger)?;

  time::loop_at_frequency(60.0, |loop_ctx| {
    while let Some(event) = window.next_event() {
      if let window::Event::CloseRequested = event {
        loop_ctx.stop();
      }
    }
  });

  Ok(())
}
