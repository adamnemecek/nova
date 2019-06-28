// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A two-dimensional size.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Size<T: ScalarNum> {
  /// Width component of the size.
  pub width: T,

  /// Height component of the size.
  pub height: T,
}

impl<T: ScalarNum> Size<T> {
  /// Creates a new size with the given width and height.
  pub fn new(width: T, height: T) -> Self {
    Size { width, height }
  }
}

impl<T: ScalarNum> Default for Size<T> {
  fn default() -> Self {
    Size::new(T::zero(), T::zero())
  }
}

impl<T: ScalarNum> From<(T, T)> for Size<T> {
  fn from(tuple: (T, T)) -> Self {
    Size::new(tuple.0, tuple.1)
  }
}

impl<T: ScalarNum> fmt::Debug for Size<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({:?}, {:?})", &self.width, &self.height)
  }
}

impl<T: ScalarNum> ops::Mul<T> for Size<T> {
  type Output = Self;

  fn mul(self, multiplier: T) -> Self {
    Size {
      width: self.width * multiplier,
      height: self.height * multiplier,
    }
  }
}
