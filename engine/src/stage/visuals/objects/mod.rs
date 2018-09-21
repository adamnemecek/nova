// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `drawing` module draws stage objects onto the screen.
//!
//! The `Sorter` system sorts all objects on the stage into draw order, so
//! that the closest object to the camera is drawn last. Sorted entities are
//! stored in the `State` resource which is used along with the `Settings`
//! resource when rendering.

use super::*;
use stage::objects::*;

mod animator;
mod draw;
mod sorter;

pub use self::animator::*;
pub use self::draw::*;
pub use self::sorter::*;

/// Animation state of an object on the stage.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Sprite {
  /// Cell of the atlas to draw.
  pub cell: Vector2<u16>,
  /// Offset from the object's position to draw the sprite.
  pub offset: Vector2<f32>,
  /// Whether the sprite should be drawn horizontally flipped.
  pub hflip: bool,
}

impl Default for Sprite {
  fn default() -> Self {
    Sprite {
      cell: Vector2::zeros(),
      offset: Vector2::zeros(),
      hflip: false,
    }
  }
}

/// Component that stores data about an object's shadow.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Shadow {
  /// Size of the shadow in pixels.
  pub size: Vector2<f32>,
}

impl Default for Shadow {
  fn default() -> Self {
    Shadow {
      size: Vector2::zeros(),
    }
  }
}

/// Component storing the animation state of an object on the stage.
#[derive(Default, Component, Debug)]
#[storage(BTreeStorage)]
pub struct AnimationState {
  /// Index of the animation in the object template.
  pub animation: usize,
  /// Index of the sequence in the animation.
  pub sequence: usize,
  /// Index of the frame in the sequence.
  pub frame: usize,
  /// Time in seconds elapsed since this frame began.
  pub frame_elapsed: f64,
}

/// Raw bytes of `circle.png`.
const CIRCLE_PNG: &[u8] = include_bytes!("circle.png");

/// Sets up object rendering for the given world.
pub fn setup<'a, 'b>(world: &mut World, systems: &mut DispatcherBuilder<'a, 'b>) {
  world.register::<Sprite>();
  world.register::<Shadow>();
  world.register::<AnimationState>();

  world.add_resource(DrawState::default());

  world.add_resource(DrawSettings {
    scale: 2.0,
    shadow_image: graphics::Image::new(CIRCLE_PNG).expect("could not load circle.png"),
  });

  systems.add(Animator, "stage::visuals::objects::Animator", &[]);
  systems.add(Sorter, "stage::visuals::objects::Sorter", &[]);
}

/// Adds components to the entity for object visuals.
pub fn build_entity<'a>(builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
  builder
    .with(Sprite::default())
    .with(Shadow::default())
    .with(AnimationState::default())
}
