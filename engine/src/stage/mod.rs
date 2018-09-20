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

mod camera;
mod direction;
mod motion;

pub use self::{camera::*, direction::*, motion::*};

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

/// Sets up the stage for the given world.
pub fn setup<'a, 'b>(world: &mut World, systems: &mut DispatcherBuilder<'a, 'b>) {
  world.register::<Position>();
  world.register::<Velocity>();

  world.add_resource(Camera::default());

  systems.add(Mover, "stage::Mover", &[]);

  objects::setup(world, systems);
  actors::setup(world, systems);
}

/// Renders the stage.
pub fn render(world: &mut World, core: &mut Core) {
  objects::render(world, core);
}
