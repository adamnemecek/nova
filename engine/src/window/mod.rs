// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod events;

pub use winit::CreationError;

use self::events::Event;
use crate::math::Size;
use crossbeam::channel;
use std::thread;

/// Represents a platform-specfic window.
pub struct Window {
  /// Raw winit window structure.
  raw: winit::Window,
  /// Receiver that gets sets of events.
  recv_events: channel::Receiver<Vec<Event>>,
  /// Size of the window in pixels.
  size: Size<u32>,
  /// Whether the user has requested the window be closed.
  closing: bool,
}

impl Window {
  /// Creates a new platform-specific window.
  pub fn new() -> Result<Window, CreationError> {
    let (send_window, recv_window) = channel::bounded(0);
    let (send_events, recv_events) = channel::bounded(0);

    thread::spawn(move || {
      let events_loop = winit::EventsLoop::new();

      let result = winit::WindowBuilder::new()
        .with_title("nova")
        .build(&events_loop);

      send_window.send(result).unwrap();

      events::process(events_loop, send_events);
    });

    let window = recv_window.recv().unwrap()?;

    let size = pixel_size_of(&window);

    Ok(Window {
      recv_events,
      raw: window,
      size,
      closing: false,
    })
  }

  /// Returns `true` after the user requests closing the window.
  pub fn is_closing(&self) -> bool {
    self.closing
  }

  /// Gets the size of the window in pixels.
  pub fn size(&self) -> Size<u32> {
    self.size
  }

  /// Updates the window by processing events that have occured since the last
  /// update.
  pub fn update(&mut self) {
    while let Ok(events) = self.recv_events.try_recv() {
      for event in events {
        match event {
          Event::CloseRequested => {
            self.closing = true;
          }

          Event::Resized => {
            self.size = pixel_size_of(&self.raw);
          }
        }
      }
    }
  }
}

// Implement `AsRef` to expose a reference to the raw winit window.
impl AsRef<winit::Window> for Window {
  fn as_ref(&self) -> &winit::Window {
    &self.raw
  }
}

/// Determines the size of a window in pixels.
fn pixel_size_of(window: &winit::Window) -> Size<u32> {
  let size = window
    .get_inner_size()
    .expect("window destroyed")
    .to_physical(window.get_hidpi_factor());

  Size::new(size.width.round() as u32, size.height.round() as u32)
}
