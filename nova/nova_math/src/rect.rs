// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Point2, ScalarNum, Size};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect<T: ScalarNum> {
  pub start: Point2<T>,
  pub end: Point2<T>,
}

impl<T: ScalarNum> Rect<T> {
  pub fn width(&self) -> T {
    self.end.x - self.start.x
  }

  pub fn height(&self) -> T {
    self.end.y - self.start.y
  }
}
