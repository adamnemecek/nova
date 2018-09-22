// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `objects` module defines systems and components for “objects” on the
//! stage.
//!
//! Objects are entities represented by a sprite that have a physical presence
//! on the stage. Objects are animated by the `Animator` system, supporting
//! animations that vary in either four or eight directions.
//!
//! An object entity can be built from a `Template`, which can be loaded from a
//! YAML asset file.

use super::*;

mod animation;
mod template;

pub use self::animation::*;
pub use self::template::*;

/// Component that stores state for an object on the stage.
///
/// Objects are entities with a physical presence on the stage, represented by
/// a sprite.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Object {
  /// Template the object was created from.
  pub template: Arc<Template>,
  /// Direction the object is facing.
  pub facing: Vector3<f32>,
}

/// Initializes objects for the given engine context.
pub fn init(ctx: &mut engine::Context) {
  engine::add_storage::<Object>(ctx);
}

/// Adds components to the entity for an object with the given `template`.
pub fn build_entity<'a>(template: Arc<Template>, builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
  builder
    .with(Position::default())
    .with(Velocity::default())
    .with(Object {
      template: template.clone(),
      facing: Vector3::y(),
    })
}
