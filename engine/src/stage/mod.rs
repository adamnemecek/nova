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
pub mod objects;
pub mod rendering;

pub mod direction;
pub mod motion;

pub use self::direction::CompassDirection;
pub use self::motion::Motion;

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

/// Component that stores the velocity of an entity on the stage.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Velocity {
  pub vector: Vector3<f32>,
}

// Sets the default velocity to zero.
impl Default for Velocity {
  fn default() -> Self {
    Velocity {
      vector: Vector3::zeros(),
    }
  }
}

/// Sets up stage components, resources, and systems.
pub fn setup<'a, 'b>(core: &mut Core, dispatch: &mut DispatcherBuilder<'a, 'b>) {
  core.world.register::<Position>();
  core.world.register::<Velocity>();

  dispatch.add(Motion, "stage::Motion", &[]);

  rendering::setup(core, dispatch);
  objects::setup(core, dispatch);
  actors::setup(core, dispatch);
}
