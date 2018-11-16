// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod swapchain;

mod events;
mod surface;

pub use self::events::{Event, EventSource};
pub use self::surface::Surface;
pub use self::swapchain::Swapchain;
pub use winit::CreationError;

use super::backend;
use crate::math::Size;
use std::sync::Arc;

/// Represents a platform-specfic window.
pub struct Window {
  /// Raw winit window structure.
  raw: winit::Window,
  /// Surface created from the window. Stored in an `Option` so that a renderer
  /// can eventually take ownership.
  surface: Option<Surface>,
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
  pub fn new(backend: &Arc<backend::Instance>) -> Result<(Window, EventSource), CreationError> {
    let events_loop = winit::EventsLoop::new();

    let raw = winit::WindowBuilder::new()
      .with_title("nova")
      .build(&events_loop)?;

    let size = pixel_size_of(&raw);

    let mut window = Window {
      raw,
      surface: None,
      size,
      closing: false,
    };

    window.surface = Some(Surface::new(backend, &window));

    Ok((window, events_loop.into()))
  }

  /// Gets a reference to the window's render surface if it has not yet been
  /// taken with [`take_surface()`].
  pub fn surface(&self) -> Option<&Surface> {
    self.surface.as_ref()
  }

  /// Returns `true` after the user requests closing the window.
  pub fn is_closing(&self) -> bool {
    self.closing
  }

  /// Gets the size of the window in pixels.
  pub fn size(&self) -> Size<u32> {
    self.size
  }

  /// Gets a reference to the window's render surface if it has not yet been
  /// taken with [`take_surface()`].
  ///
  /// Panics if the surface has already been taken.
  pub fn take_surface(&mut self) -> Surface {
    self
      .surface
      .take()
      .expect("Window surface has already been taken.")
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
