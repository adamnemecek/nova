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

pub mod render;

mod animation;
mod animator;
mod template;

pub use self::{animation::*, animator::*, render::render, template::*};

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
  /// State of the object's animation.
  pub animation: AnimationState,
}

/// Animation state of an object on the stage.
#[derive(Default)]
pub struct AnimationState {
  /// Index in the object template's animation list of the animation to play.
  pub index: usize,
  /// Seconds elapsed in the animation.
  pub elapsed: f64,
}

/// Sets up components, resources, and systems needed for objects.
pub fn setup<'a, 'b>(core: &mut Core, dispatch: &mut DispatcherBuilder<'a, 'b>) {
  core.world.register::<Object>();

  dispatch.add(Animator, "stage::objects::Animator", &[]);

  render::setup(core, dispatch);
}

/// Adds components to the entity for an object with the given `template`.
pub fn build_entity<'a>(template: Arc<Template>, builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
  builder
    .with(graphics::Sprite::new(template.atlas.clone()))
    .with(Position::default())
    .with(Velocity::default())
    .with(Object {
      template: template.clone(),
      facing: Vector3::y(),
      animation: AnimationState::default(),
    })
}
