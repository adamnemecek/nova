use nova::math::Size;
use nova::time;
use nova::window;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
  let window = window::open(window::Options {
    size: Size::new(2560, 1440),
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
