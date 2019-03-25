// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `math` module exposes common functions and types for working with
//! scalar numbers, vectors, matrices, and geometric dimensions.

mod rect;
mod size;

pub use self::rect::Rect;
pub use self::size::Size;
pub use nalgebra::Matrix4;
pub use nalgebra::Scalar;
pub use nalgebra::{Point2, Point3};
pub use nalgebra::{Vector2, Vector3, Vector4};
pub use num_traits::Num;

use std::ops::Range;

/// Common trait for scalar numbers. Types with this trait can be used in math
/// structures like [`Vector2`] or [`Rect`].
///
/// It is automatically implemented for types that implement [`Scalar`] and
/// [`Num`], including all primitive numeric types.
pub trait ScalarNum: Scalar + Num {}

impl<T: Scalar + Num> ScalarNum for T {}

#[inline]
pub fn clamp<T: PartialOrd>(input: T, range: Range<T>) -> T {
  if input < range.start {
    range.start
  } else if input > range.end {
    range.end
  } else {
    input
  }
}
