// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// One of the eight compass directions.
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

/// Total number of compass directions.
pub const COMPASS_DIRECTION_COUNT: usize = CompassDirection::SouthEast as usize + 1;

/// Names of each of the compass directions.
pub const COMPASS_DIRECTION_NAMES: [&'static str; COMPASS_DIRECTION_COUNT] = [
  "south",
  "southwest",
  "west",
  "northwest",
  "north",
  "northeast",
  "east",
  "southeast",
];

impl CompassDirection {
  /// Gets the nearest compass direction to the given vector.
  pub fn nearest(vec: Vector2<f32>) -> CompassDirection {
    // TODO: Implement for all 8 directions.
    Self::nearest_cardinal(vec)
  }

  /// Gets the nearest cardinal direction to the given vector.
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
