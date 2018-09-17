// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

pub mod system;

pub use self::system::MotionSystem;

/// Component that stores the position of an entity on the stage.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Position {
  pub point: Point3<f32>,
}

impl Default for Position {
  fn default() -> Self {
    Position {
      point: Point3::new(0.0, 0.0, 0.0),
    }
  }
}

/// Component that stores the velocity of an entity on the stage.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Velocity {
  pub vector: Vector3<f32>,
}

impl Default for Velocity {
  fn default() -> Self {
    Velocity {
      vector: Vector3::zeros(),
    }
  }
}
