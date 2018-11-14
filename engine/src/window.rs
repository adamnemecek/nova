// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use winit::CreationError;

use crate::math::Size;
use std::sync::Arc;

/// Represents a platform-specfic window.
pub struct Window {
  /// Events loop the window was created with.
  events_loop: winit::EventsLoop,
  /// Raw winit window structure.
  raw: winit::Window,
  /// Size of the window in pixels.
  size: Size<u32>,
  /// Whether the user has requested the window be closed.
  closing: bool,
}

impl Window {
  /// Creates a new platform-specific window.
  pub fn new() -> Result<Window, CreationError> {
    let events_loop = winit::EventsLoop::new();

    let window = winit::WindowBuilder::new()
      .with_title("nova")
      .build(&events_loop)?;

    let size = pixel_size_of(&window);

    Ok(Window {
      events_loop,
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
    let mut closing = false;
    let mut resized = false;

    self.events_loop.poll_events(|event| match event {
      winit::Event::WindowEvent { event, .. } => match event {
        winit::WindowEvent::CloseRequested => {
          closing = true;
        }

        winit::WindowEvent::Resized(_) => {
          resized = true;
        }

        _ => {}
      },

      _ => {}
    });

    if closing {
      self.closing = true;
    }

    if resized {
      self.size = pixel_size_of(&self.raw);
    }
  }
}

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
