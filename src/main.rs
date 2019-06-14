use nova::component::{Component, NullStorage};
use nova::math::Size;
use nova::time;
use nova::window::{Window, WindowEvent, WindowOptions};
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
  let window = Window::new(WindowOptions {
    title: "tvb".into(),
    size: Size::new(2560, 1440),
    resizable: false,
  });

  time::loop_at_frequency(60.0, |ctx| {
    while let Some(event) = window.next_event() {
      if let WindowEvent::CloseRequested = event {
        ctx.stop();
      }
    }
  });

  Ok(())
}

#[derive(Default)]
struct Test;

impl Component for Test {
  type Storage = NullStorage<Self>;
}
