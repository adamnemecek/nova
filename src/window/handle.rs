// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Handle to a platform-specific window.
///
/// When this structure is dropped, the window is closed.
pub struct Handle {
  window: Arc<winit::Window>,
  events: channel::Receiver<winit::WindowEvent>,
}

impl Handle {
  pub(crate) fn new(window: winit::Window, events: channel::Receiver<winit::WindowEvent>) -> Self {
    Self { window: Arc::new(window), events }
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

  /// Returns a reference to the underlying winit window.
  pub(crate) fn as_winit(&self) -> &Arc<winit::Window> {
    &self.window
  }
}
