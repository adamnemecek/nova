// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::backend::Backend;
use crate::prelude::*;
use gfx_hal as hal;

pub struct Canvas<'a> {
  pub(super) size: Vector2<u32>,
  pub(super) encoder: hal::command::RenderPassInlineEncoder<'a, Backend, hal::command::Primary>,
}

impl<'a> Canvas<'a> {
  /// Gets the size of the canvas in pixels.
  pub fn size(&self) -> Vector2<u32> {
    self.size
  }
}
