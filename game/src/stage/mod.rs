// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `stage` module implements the _stage_, a physical world intended for
//! top-down 2D games.
//!
//! An entity “on the stage” has a `Position` component that stores its position
//! in game world coordinates. Coordinates are three-dimensional: the x-axis is
//! from “west” to “east” or left to right on the screen, the y-axis is from
//! “north” to “south” or top to bottom on the screen, and the z-axis is from
//! “down” to “up” which indicates altitude.
//!
//! The `motion` module adds the `Velocity` component and a `Mover` system that
//! moves entities' positions each run according to their velocities.
//!
//! For sprites that can only face four or eight directions, the `direction`
//! module implements `CompassDirection` with conversion from vectors.

use crate::prelude::*;

pub mod actors;
pub mod graphics;
pub mod objects;

mod direction;
mod motion;

pub use self::direction::*;
pub use self::motion::*;

/// Component that stores the position of an entity on the stage.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Position {
  pub point: Point3<f32>,
}

// Sets the default position to all zeros.
impl Default for Position {
  fn default() -> Self {
    Position {
      point: Point3::new(0.0, 0.0, 0.0),
    }
  }
}

/// Initializes the stage in the given engine context.
pub fn init(ctx: &mut engine::Context) {
  engine::add_storage::<Position>(ctx);
  engine::add_storage::<Velocity>(ctx);

  engine::add_system(ctx, Mover, "stage::Mover", &[]);

  objects::init(ctx);
  actors::init(ctx);
}
