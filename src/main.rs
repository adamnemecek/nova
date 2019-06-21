use nova::graphics;
use nova::graphics::renderer;
use nova::log;
use nova::time;
use nova::window;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
  // Create a terminal logger and set it as the global default.
  let logger = log::terminal_compact();

  log::set_global_logger(&logger);

  // Create a graphics context and background loader.
  let graphics = graphics::Context::new(&logger)?;
  let loader = graphics::Loader::new(&graphics);

  // Open a window.
  let window = window::open(window::Options {
    size: (2560.0, 1440.0).into(),
    resizable: true,
    ..Default::default()
  });

  // Start the renderer.
  let renderer = renderer::start(&graphics, &window, &loader, &logger)?;

  // Run the main game loop 60 times per second.
  time::loop_at_frequency(60.0, |main_loop| {
    while let Some(event) = window.next_event() {
      match event {
        // When the user tries to close the window, exit the game loop.
        window::Event::CloseRequested => {
          log::info!(logger, "close requested");

          return main_loop.stop();
        }

        // When the window is resized, resize the render surface as well.
        window::Event::Resized => {
          renderer.resize_surface(window.size());
        }
      }
    }
  });

  // Shut down the renderer before exiting to clean up resources.
  renderer.shut_down();

  Ok(())
}
