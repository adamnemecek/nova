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
    Size::new(self.width(), self.height())
  }

  pub fn width(&self) -> T {
    self.x2 - self.x1
  }

  pub fn height(&self) -> T {
    self.y2 - self.y1
  }
}

impl<T: ScalarNum> From<Size<T>> for Rect<T> {
  fn from(size: Size<T>) -> Self {
    Self {
      x1: T::zero(),
      y1: T::zero(),
      x2: size.width,
      y2: size.height,
    }
  }
}
