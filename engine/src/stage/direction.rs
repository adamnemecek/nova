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

pub const COMPASS_DIRECTION_COUNT: usize = CompassDirection::SouthEast as usize + 1;

impl CompassDirection {
  pub fn nearest(vec: Vector2<f32>) -> CompassDirection {
    Self::nearest_cardinal(vec)
  }

  pub fn nearest_cardinal(vec: Vector2<f32>) -> CompassDirection {
    if vec.x.abs() > vec.y.abs() {
      if vec.x < 0.0 {
        CompassDirection::West
      } else {
        CompassDirection::East
      }
    } else {
      if vec.y < 0.0 {
        CompassDirection::North
      } else {
        CompassDirection::South
      }
    }
  }
}
