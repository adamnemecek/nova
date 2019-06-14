// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod options;

pub use winit::ElementState as ButtonState;
pub use winit::VirtualKeyCode as KeyCode;
pub use winit::{MouseButton, WindowEvent};
pub use self::options::WindowOptions;

use nova_core::channel;
use nova_core::math::Size;
use std::thread;

pub struct Window {
  window: winit::Window,
  events: channel::Receiver<WindowEvent>,
}

impl Window {
  pub fn new(options: WindowOptions) -> Self {
    let (event_sender, events) = channel::unbounded();
    let (window_sender, window) = channel::bounded(0);

    thread::spawn(move || {
      let mut events_loop = winit::EventsLoop::new();

      let window = winit::WindowBuilder::new()
        .with_title(options.title)
        .with_resizable(options.resizable)
        .with_dimensions(
          winit::dpi::PhysicalSize::new(options.size.width.into(), options.size.height.into())
            .to_logical(events_loop.get_primary_monitor().get_hidpi_factor()),
        )
        .build(&events_loop)
        .expect("Could not create window");

      if window_sender.send(window).is_err() {
        return;
      }

      drop(window_sender);

      events_loop.run_forever(|event| {
        if let winit::Event::WindowEvent { event, .. } = event {
          if event_sender.send(event).is_err() {
            return winit::ControlFlow::Break;
          }
        }

        winit::ControlFlow::Continue
      });
    });

    let window = window.recv().expect("Could not create window");

    Window { window, events }
  }

  pub fn dpi(&self) -> f64 {
    self.window.get_hidpi_factor()
  }

  pub fn size(&self) -> Size<u32> {
    let (width, height): (u32, u32) = self
      .window
      .get_inner_size()
      .expect("Could not get window size")
      .to_physical(self.dpi())
      .into();

    Size::new(width, height)
  }

  pub fn next_event(&self) -> Option<WindowEvent> {
    self.events.try_recv().ok()
  }
}
