// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Point2, ScalarNum};

/// Represents a two-dimensional rectangle in space.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect<T: ScalarNum> {
  /// The start point of the rectangle, its top-left corner.
  pub start: Point2<T>,

  /// The end point of the rectangle, its bottom-right corner.
  pub end: Point2<T>,
}

impl<T: ScalarNum> Rect<T> {
  /// Returns the width of the rectangle.
  pub fn width(&self) -> T {
    self.end.x - self.start.x
  }

  /// Returns the height of the rectangle.
  pub fn height(&self) -> T {
    self.end.y - self.start.y
  }
}
