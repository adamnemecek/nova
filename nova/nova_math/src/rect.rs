// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Point2, ScalarNum, Size};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect<T> {
  pub x1: T,
  pub y1: T,
  pub x2: T,
  pub y2: T,
}

impl<T: ScalarNum> Rect<T> {
  pub fn new(position: Point2<T>, size: Size<T>) -> Self {
    Self {
      x1: position.x,
      y1: position.y,
      x2: position.x + size.width,
      y2: position.y + size.height,
    }
  }

  pub fn position(&self) -> Point2<T> {
    Point2::new(self.x1, self.y1)
  }

  pub fn size(&self) -> Size<T> {
    Size::new(self.x2 - self.x1, self.y2 - self.y1)
  }
}

impl Rect<f32> {
  pub const fn unit() -> Self {
    Rect {
      x1: 0.0,
      y1: 0.0,
      x2: 1.0,
      y2: 1.0,
    }
  }
}
