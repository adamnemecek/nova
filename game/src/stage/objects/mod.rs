//! The `objects` module implements _stage objects_.
//!
//! An object is an entity that represents a single physical object on the
//! stage. Each object is created from a `Template` which can be loaded from a
//! YAML file.

use super::{Position, Velocity};
use crate::prelude::*;
use std::sync::Arc;

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
pub fn build_entity<'a>(
  template: Arc<Template>,
  builder: impl EntityBuilder + 'a,
) -> impl EntityBuilder + 'a {
  builder
    .with(Position::default())
    .with(Velocity::default())
    .with(Object {
      template: template.clone(),
      facing: Vector3::y(),
    })
}
