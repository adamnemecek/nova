// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `render` module renders objects onto the screen.
//!
//! The `Sorter` system sorts all objects on the stage into draw order, so
//! that the closest object to the camera is drawn last. Sorted entities are
//! stored in the `State` resource which is used along with the `Settings`
//! resource when rendering.

use super::*;
use ggez::graphics::DrawParam;

mod sorter;

pub use self::sorter::*;

/// State of object rendering.
#[derive(Default)]
pub struct State {
  /// Entities on the stage that have an `Object` component, in order from
  /// background to foreground.
  pub entities: Vec<Entity>,
}

/// Settings for object rendering.
pub struct Settings {
  /// Global scale for drawing objects.
  pub scale: f32,
  /// Texture to use for drawing shadows. Not provided by default.
  pub shadow_texture: Option<graphics::Image>,
}

/// Sets up object rendering for the given world.
pub fn setup<'a, 'b>(world: &mut World, systems: &mut DispatcherBuilder<'a, 'b>) {
  world.add_resource(State::default());

  world.add_resource(Settings {
    scale: 2.0,
    shadow_texture: None,
  });

  systems.add(Sorter, "stage::objects::render::Sorter", &[]);
}

/// Renders all the objects on the stage.
pub fn render(world: &mut World, core: &mut Core) {
  let state = world.read_resource::<State>();
  let settings = world.read_resource::<Settings>();

  let viewport = world.read_resource::<core::Viewport>();
  let positions = world.read_storage::<Position>();
  let objects = world.read_storage::<Object>();
  let sprites = world.read_storage::<graphics::Sprite>();

  // Determine position of camera.
  let camera_pos = match world.read_resource::<Camera>().target {
    CameraTarget::Position(pos) => pos,
    CameraTarget::Entity(entity) => {
      if let Some(pos) = positions.get(entity) {
        Point2::new(pos.point.x, pos.point.y)
      } else {
        Point2::new(0.0, 0.0)
      }
    }
  };

  // Calculate the offset in drawing needed for the camera's position.
  let global_offset =
    Point2::new(viewport.width, viewport.height) / settings.scale / 2.0 - camera_pos;

  // Apply scale transform.
  ggez::graphics::push_transform(&mut core.ctx, Some(Matrix4::new_scaling(settings.scale)));
  ggez::graphics::apply_transformations(&mut core.ctx).expect("could not scale for stage draw");

  // Draw object shadows.
  if let Some(ref shadow_image) = settings.shadow_texture {
    for entity in &state.entities {
      let position = &positions.get(*entity).unwrap().point;
      let size = &objects.get(*entity).unwrap().template.shadow_size;
      let image_size = shadow_image.size();

      shadow_image
        .draw(
          core,
          DrawParam::default()
            .color(ggez::graphics::Color::new(0.0, 0.0, 0.0, 0.2))
            .scale(Vector2::new(size.0 / image_size.x, size.1 / image_size.y))
            .dest(
              Point2::new(position.x - size.0 / 2.0, position.y - size.1 / 2.0) + global_offset,
            ),
        )
        .expect("could not draw sprite");
    }
  }

  // Draw object sprites.
  for entity in &state.entities {
    let sprite = sprites.get(*entity).unwrap();
    let position = positions.get(*entity).unwrap();

    let scale = sprite.scale;
    let mut offset = sprite.offset - sprite.atlas.cell_origin;

    offset.x *= scale.x;
    offset.y *= scale.y;

    let params = DrawParam::default()
      .src(sprite.atlas.get(sprite.cell))
      .scale(scale)
      .dest(
        Point2::new(position.point.x, position.point.y - position.point.z) + offset + global_offset,
      );

    let image = &sprite.atlas.image;

    image.draw(core, params).expect("could not draw sprite");
  }

  ggez::graphics::pop_transform(&mut core.ctx);
  ggez::graphics::apply_transformations(&mut core.ctx)
    .expect("could not restore scale after stage draw");
}
