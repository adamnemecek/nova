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
pub use nalgebra::{Transform2, Transform3};
pub use nalgebra::{Vector2, Vector3, Vector4};
pub use num_traits::Num;

use std::ops;

/// Common trait for scalar numbers. Types with this trait can be used in math
/// structures like [`Vector2`] or [`Rect`].
///
/// It is automatically implemented for types that implement [`Scalar`] and
/// [`Num`], including all primitive numeric types.
pub trait ScalarNum: Scalar + Num {}

impl<T: Scalar + Num> ScalarNum for T {}

/// Returns the given `input` value constrained to be within the given `bounds`.
///
/// The returned value will be no less than the start bound if it exists and no
/// more than the end bound if it exists. Whether the bounds are inclusive or
/// exclusive is ignored.
#[inline]
pub fn clamp<T: Copy + PartialOrd>(input: T, bounds: impl ops::RangeBounds<T>) -> T {
  match bounds.start_bound() {
    ops::Bound::Excluded(value) | ops::Bound::Included(value) if input < *value => *value,
    _ => match bounds.end_bound() {
      ops::Bound::Excluded(value) | ops::Bound::Included(value) if input > *value => *value,
      _ => input,
    },
  }
}
