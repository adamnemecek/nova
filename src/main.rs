use nova::graphics;
use nova::log;
use nova::time;
use nova::window;
use std::error::Error;
use std::thread;

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

  // Start the renderer on a separate thread.
  //start_renderer(&graphics, &window, &logger);

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

/// Starts a renderer loop on a background thread.
///
/// TODO: This should maybe be contained in a `Renderer` type.
fn start_renderer(graphics: &graphics::Context, window: &window::Handle, logger: &log::Logger) {
  let logger = logger.clone();

  // Create resources needed for rendering.
  let mut surface = graphics::Surface::new(&graphics, &window);
  let acquire_semaphore = graphics::Semaphore::new(&graphics);

  // Run the renderer 60 times per second on a background thread.
  thread::spawn(move || {
    time::loop_at_frequency(60.0, |loop_ctx| {
      // Acquire a backbuffer from the surface to render to.
      let backbuffer = match surface.acquire(&acquire_semaphore) {
        Ok(backbuffer) => backbuffer,

        Err(err) => {
          log::error!(logger,
            "failed to acquire surface backbuffer";
            "cause" => log::Display(err),
          );

          return loop_ctx.stop();
        }
      };

      // TODO: Render

      // Present the rendered backbuffer.
      if let Err(err) = backbuffer.present(&[&acquire_semaphore]) {
        log::error!(logger,
          "failed to present surface backbuffer";
          "cause" => log::Display(err),
        );

        return loop_ctx.stop();
      }
    });
  });
}
