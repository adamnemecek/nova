// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Point2, ScalarNum, Size};

/// Definition of a two-dimensional rectangle.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rect<N: ScalarNum> {
  /// Position of the rectangle's top-left corner.
  pub pos: Point2<N>,
  /// Size of the rectangle.
  pub size: Size<N>,
}

impl<N: ScalarNum> Rect<N> {
  /// Creates a new rect with the given dimensions.
  pub fn new(x: N, y: N, width: N, height: N) -> Self {
    Rect {
      pos: Point2::new(x, y),
      size: Size::new(width, height),
    }
  }
}

// Implement `Default` to provide a rectangle with all zeros.
impl<N: ScalarNum> Default for Rect<N> {
  fn default() -> Self {
    Rect {
      pos: Point2::origin(),
      size: Size::default(),
    }
  }
}
