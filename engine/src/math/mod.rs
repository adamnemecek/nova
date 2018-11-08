// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod rect;

pub use nalgebra::Real;

pub mod algebra {
  pub use nalgebra::Matrix4;
  pub use nalgebra::{Vector2, Vector3, Vector4};
}

pub mod geometry {
  pub use super::rect::*;
  pub use nalgebra::{Point2, Point3};
}
