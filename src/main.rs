use nova::graphics;
use nova::log;
use nova::time;
use nova::window;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
  // Create a terminal logger and set it as the global default.
  let logger = log::terminal_compact();

  log::set_global_logger(&logger);

  // Create a graphics context.
  let graphics = graphics::Context::new(&logger)?;

  // Open a window.
  let window = window::open(window::Options {
    size: (2560.0, 1440.0).into(),
    ..Default::default()
  });

  // Create a surface from the window for rendering.
  let _surface = graphics::Surface::new(&graphics, &window);

  // Run the main game loop 60 times per second.
  time::loop_at_frequency(60.0, |loop_ctx| {
    while let Some(event) = window.next_event() {
      // When the user requests the window to close, exit the main game loop.
      if let window::Event::CloseRequested = event {
        log::info!(logger, "close requested");

        return loop_ctx.stop();
      }
    }
  });

  Ok(())
}
