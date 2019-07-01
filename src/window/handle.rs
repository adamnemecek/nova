// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Handle to a platform-specific window.
///
/// When this structure is dropped, the window is closed.
pub struct Handle {
  window: Arc<winit::Window>,
  events: mpsc::UnboundedReceiver<winit::WindowEvent>,
}

impl Handle {
  pub(crate) fn new(
    window: winit::Window,
    events: mpsc::UnboundedReceiver<winit::WindowEvent>,
  ) -> Self {
    Self { window: Arc::new(window), events }
  }

  /// Returns the next window event if one is available or `None` if there is no
  /// available event.
  pub fn next_event(&mut self) -> Option<Event> {
    let event = self.events.try_next().ok()??;

    Some(match event {
      winit::WindowEvent::CloseRequested => Event::CloseRequested,
      winit::WindowEvent::Resized(_) => Event::Resized,

      _ => return None,
    })
  }

  /// Returns a reference to the underlying winit window.
  pub(crate) fn as_winit(&self) -> &Arc<winit::Window> {
    &self.window
  }
}
