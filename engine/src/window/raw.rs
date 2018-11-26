// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::math::Size;
use derive_more::*;
use std::sync::Arc;

#[derive(Clone, Deref)]
pub struct RawHandle(Arc<winit::Window>);

impl RawHandle {
  pub fn get_inner_size(&self) -> Size<u32> {
    let size = self
      .0
      .get_inner_size()
      .expect("That window has been closed.");

    physical_size(size, self.0.get_hidpi_factor())
  }

  pub fn set_inner_size(&self, size: Size<u32>) {
    self
      .0
      .set_inner_size(logical_size(size, self.0.get_hidpi_factor()));
  }
}

// Implement `From` to convert from raw winit windows.
impl From<winit::Window> for RawHandle {
  fn from(inner: winit::Window) -> Self {
    RawHandle(Arc::new(inner))
  }
}

pub(super) fn physical_size(size: winit::dpi::LogicalSize, dpi: f64) -> Size<u32> {
  let size: (u32, u32) = size.to_physical(dpi).into();

  Size::new(size.0, size.1)
}

pub(super) fn logical_size(size: Size<u32>, dpi: f64) -> winit::dpi::LogicalSize {
  winit::dpi::LogicalSize::from_physical((size.width(), size.height()), dpi)
}
