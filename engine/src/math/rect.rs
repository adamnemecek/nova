// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::algebra::*;
use super::geometry::*;
use super::Real;
use std::fmt;

/// Struct describing a rectangle in 2D space.
#[derive(Clone, Copy, PartialEq)]
pub struct Rect<N: Real> {
  /// Position of the rectangle's top left corner.
  pub pos: Point2<N>,
  /// Size of the rectangle.
  pub size: Vector2<N>,
}

impl<N: Real> Rect<N> {
  /// Creates a new rect with the given dimensions.
  pub fn new(pos_x: N, pos_y: N, size_x: N, size_y: N) -> Self {
    Rect {
      pos: Point2::new(pos_x, pos_y),
      size: Vector2::new(size_x, size_y),
    }
  }

  /// Returns a new rect offset from this one by the given vector.
  pub fn offset(&self, vector: Vector2<N>) -> Self {
    Rect {
      pos: self.pos + vector,
      size: self.size,
    }
  }
}

impl<N: Real> Default for Rect<N> {
  fn default() -> Self {
    Rect {
      pos: Point2::origin(),
      size: Vector2::zeros(),
    }
  }
}

impl Into<ggez::graphics::Rect> for Rect<f32> {
  fn into(self) -> ggez::graphics::Rect {
    ggez::graphics::Rect::new(self.pos.x, self.pos.y, self.size.x, self.size.y)
  }
}

impl<N: Real> fmt::Debug for Rect<N> {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(
      f,
      "Rect {{ pos: ({:?}, {:?}), size: ({:?}, {:?}) }}",
      self.pos.x, self.pos.y, self.size.x, self.size.y
    )
  }
}
