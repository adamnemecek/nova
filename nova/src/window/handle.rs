// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Handle to a platform-specific window.
///
/// When this structure is dropped, the window is closed.
#[derive(Clone)]
pub struct Handle {
  window: Arc<winit::Window>,
  events: channel::Receiver<winit::WindowEvent>,
}

impl Handle {
  pub(crate) fn new(window: winit::Window, events: channel::Receiver<winit::WindowEvent>) -> Self {
    Self {
      window: Arc::new(window),
      events,
    }
  }

  /// Returns the DPI scaling factor of the window.
  ///
  /// On standard DPI screens this returns `1.0`. On high definition screens
  /// it may return `2.0` or some other multiplier.
  pub fn dpi_scaling(&self) -> f64 {
    self.window.get_hidpi_factor()
  }

  /// Returns the size of the window in pixels.
  pub fn size(&self) -> Size<f64> {
    if let Some(size) = self.window.get_inner_size() {
      Size::new(size.width, size.height) * self.dpi_scaling()
    } else {
      Size::default()
    }
  }

  /// Returns the next window event if one is available or `None` if there is no
  /// available event.
  pub fn next_event(&self) -> Option<Event> {
    self.events.try_recv().ok().and_then(|event| match event {
      winit::WindowEvent::CloseRequested => Some(Event::CloseRequested),
      winit::WindowEvent::Resized(_) => Some(Event::Resized),

      _ => None,
    })
  }
}

// Implement `AsRef` to expose a reference to the underlying winit window.
impl AsRef<winit::Window> for Handle {
  fn as_ref(&self) -> &winit::Window {
    &self.window
  }
}
