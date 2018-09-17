// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

pub enum CompassDirection {
  South,
  SouthWest,
  West,
  NorthWest,
  North,
  NorthEast,
  East,
  SouthEast,
}

pub const COUNT: usize = CompassDirection::SouthEast as usize + 1;

impl CompassDirection {
  pub fn nearest(_vec: Vector2<f32>) -> CompassDirection {
    CompassDirection::South
  }

  pub fn nearest_cardinal(_vec: Vector2<f32>) -> CompassDirection {
    CompassDirection::South
  }
}
