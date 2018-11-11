use crate::prelude::*;

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

impl Default for CompassDirection {
  fn default() -> Self {
    CompassDirection::South
  }
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
