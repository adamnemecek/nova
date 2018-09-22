// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `actors` module defines systems and components for _actors_, objects
//! that can move around and interact with the stage.
//!
//! An actor is an entity represented by an object. Each actor has a _mode_ that
//! describes the basic action it is taking such as walking or idling. The
//! `Animator` system changes each actor's current object animation to match its
//! mode.
//!
//! An actor entity can be built from a `Template` loaded from a YAML asset
//! file.

use super::*;

pub mod driving;

mod template;

pub use self::template::*;

/// Component that stores the state of an actor.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Actor {
  /// Template the actor was created from.
  pub template: Arc<Template>,
  /// Mode of the actor.
  pub mode: Mode,
}

/// One of the possible modes of an actor.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum Mode {
  None,
  Idle,
  Walk,
}

/// Total number of possible actor modes.
pub const MODE_COUNT: usize = Mode::Walk as usize + 1;

/// Initializes actors for the given engine context.
pub fn init(ctx: &mut engine::Context) {
  engine::add_storage::<Actor>(ctx);
}

/// Adds components to the entity for an actor with the given `template`.
pub fn build_entity<'a>(template: Arc<Template>, builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
  objects::build_entity(template.object.clone(), builder).with(Actor {
    template,
    mode: Mode::Idle,
  })
}
