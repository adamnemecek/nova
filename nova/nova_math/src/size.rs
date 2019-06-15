// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::ScalarNum;
use std::fmt;
use std::ops::{Div, Mul};

/// Two-dimensional size with width and height.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Size<T: ScalarNum> {
  pub width: T,
  pub height: T,
}

impl<T: ScalarNum> Default for Size<T> {
  fn default() -> Self {
    Size::new(T::zero(), T::zero())
  }
}

impl<T: ScalarNum> From<(T, T)> for Size<T> {
  fn from((width, height): (T, T)) -> Self {
    Self { width, height }
  }
}

impl<T: ScalarNum> Size<T> {
  /// Creates a new size with the given width and height.
  pub fn new(width: T, height: T) -> Self {
    Size { width, height }
  }
}

impl<T: ScalarNum> fmt::Debug for Size<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({:?}, {:?})", &self.width, &self.height)
  }
}
