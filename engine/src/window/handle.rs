// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::math::Size;
use std::sync::Arc;

#[derive(Clone)]
pub struct Handle(Arc<winit::Window>);

impl Handle {
  pub fn set_title(&self, title: &str) {
    self.0.set_title(title);
  }

  pub fn set_resizable(&self, resizable: bool) {
    self.0.set_resizable(resizable);
  }

  pub fn get_size(&self) -> Size<u32> {
    let size = self
      .0
      .get_inner_size()
      .expect("That window has been closed.");

    physical_size(size, self.0.get_hidpi_factor())
  }

  pub fn set_size(&self, size: Size<u32>) {
    self
      .0
      .set_inner_size(logical_size(size, self.0.get_hidpi_factor()));
  }

  pub fn set_fullscreen(&self, fullscreen: bool) {
    self.0.set_fullscreen(match fullscreen {
      true => Some(self.0.get_current_monitor()),
      false => None,
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

pub(super) fn physical_size(size: winit::dpi::LogicalSize, dpi: f64) -> Size<u32> {
  let size: (u32, u32) = size.to_physical(dpi).into();

  Size::new(size.0, size.1)
}

pub(super) fn logical_size(size: Size<u32>, dpi: f64) -> winit::dpi::LogicalSize {
  winit::dpi::LogicalSize::from_physical((size.width(), size.height()), dpi)
}
