// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;

/// Struct describing a rectangle in 2D space.
#[derive(Clone, Copy)]
pub struct Rect {
  /// Position of the rectangle's top left corner.
  pub position: Point2<f32>,
  /// Size of the rectangle.
  pub size: Vector2<f32>,
}

impl Rect {
  /// Offsets the rect by a given vector.
  pub fn offset(&self, vector: Vector2<f32>) -> Rect {
    Rect {
      position: self.position + vector,
      size: self.size,
    }
  }
}

impl Default for Rect {
  fn default() -> Rect {
    Rect {
      position: Point2::origin(),
      size: Vector2::zeros(),
    }
  }
}
