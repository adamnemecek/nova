// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod events;

pub use self::events::{Event, EventSource};
pub use winit::CreationError;

use crate::math::Size;

/// Represents a platform-specfic window.
pub struct Window {
  /// Raw winit window structure.
  raw: winit::Window,
  /// Size of the window in pixels.
  size: Size<u32>,
  /// Whether the user has requested the window be closed.
  closing: bool,
}

impl Window {
  /// Creates a new platform-specific window.
  ///
  /// This function returns a `Window` and an [`EventSource`] which can be used
  /// to poll window events.
  pub fn new() -> Result<(Window, EventSource), CreationError> {
    let events_loop = winit::EventsLoop::new();

    let raw = winit::WindowBuilder::new()
      .with_title("nova")
      .build(&events_loop)?;

    let size = pixel_size_of(&raw);

    let window = Window {
      raw,
      size,
      closing: false,
    };

    Ok((window, events_loop.into()))
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
  pub fn update<'a>(&mut self, events: impl IntoIterator<Item = &'a Event>) {
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
