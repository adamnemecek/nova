// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{logical_size, physical_size};
use crate::math::Size;
use std::sync::Arc;

/// A handle to a [`Window`].
#[derive(Clone)]
pub struct Handle(Arc<winit::Window>);

impl Handle {
  /// Sets the title of the window.
  pub fn set_title(&self, title: &str) {
    self.0.set_title(title);
  }

  /// Sets whether the window is resizable.
  pub fn set_resizable(&self, resizable: bool) {
    self.0.set_resizable(resizable);
  }

  /// Gets the current size of the window.
  pub fn get_size(&self) -> Size<u32> {
    let size = self
      .0
      .get_inner_size()
      .expect("That window has been closed.");

    physical_size(size, self.0.get_hidpi_factor())
  }

  /// Sets the current size of the window.
  pub fn set_size(&self, size: Size<u32>) {
    self
      .0
      .set_inner_size(logical_size(size, self.0.get_hidpi_factor()));
  }

  /// Sets whether or not the window is fullscreen.
  ///
  /// The window is made fullscreen on whichever monitor it is open on.
  pub fn set_fullscreen(&self, fullscreen: bool) {
    self.0.set_fullscreen(if fullscreen {
      Some(self.0.get_current_monitor())
    } else {
      None
    });
  }
}

// Implement `From` to convert from raw winit windows.
impl From<winit::Window> for Handle {
  fn from(inner: winit::Window) -> Self {
    Handle(Arc::new(inner))
  }
}

// Implement `AsRef` to expose the raw winit window.
impl AsRef<winit::Window> for Handle {
  fn as_ref(&self) -> &winit::Window {
    &self.0
  }
}
