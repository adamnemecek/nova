// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `stage` module provides functionality for the _stage_, which is the
//! physical world of the game.
//!
//! Entities “on the stage” have a `Position` component indicating where the
//! entity is in game world coordinates. The `Motion` system will also update
//! this component for all entities with a `Velocity` component.

use super::*;

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
