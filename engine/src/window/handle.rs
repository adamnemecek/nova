// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::math::Size;
use derive_more::*;
use std::sync::Arc;

#[derive(Deref, Clone)]
pub struct Handle(Arc<winit::Window>);

impl Handle {
  pub fn calculate_inner_size(&self) -> Size<u32> {
    let size = self
      .0
      .get_inner_size()
      .expect("That window has been closed.")
      .to_physical(self.0.get_hidpi_factor());

    Size::new(size.width.round() as u32, size.height.round() as u32)
  }
}

impl From<winit::Window> for Handle {
  fn from(inner: winit::Window) -> Self {
    Handle(Arc::new(inner))
  }
}
