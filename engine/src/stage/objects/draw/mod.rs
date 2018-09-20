// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `draw` module draws objects onto the screen.
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

  systems.add(Sorter, "stage::objects::draw::Sorter", &[]);
}

/// Draws all the objects on the stage.
pub fn draw(world: &mut World, canvas: &mut graphics::Canvas) {
  let state = world.read_resource::<State>();
  let settings = world.read_resource::<Settings>();

  let positions = world.read_storage::<Position>();
  let objects = world.read_storage::<Object>();

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
    Point2::new(canvas.width(), canvas.height()) / settings.scale / 2.0 - camera_pos;

  // Apply scale transform.
  canvas.push_transform(Matrix4::new_scaling(settings.scale));

  // Draw object shadows.
  if let Some(ref shadow_image) = settings.shadow_texture {
    for entity in &state.entities {
      let position = &positions.get(*entity).unwrap().point;
      let size = &objects.get(*entity).unwrap().template.shadow_size;
      let image_size = shadow_image.size();

      canvas
        .draw(
          shadow_image,
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

  // Draw objects.
  for entity in &state.entities {
    let position = positions.get(*entity).unwrap();
    let object = objects.get(*entity).unwrap();

    let animation = &object.template.animations[object.animation.index];

    if let Some(ref sequence) = animation.sequences[object.animation.sequence] {
      let atlas = &object.template.atlas;
      let frame = &sequence[object.animation.frame];

      let scale = if frame.hflip {
        Vector2::new(-1.0, 1.0)
      } else {
        Vector2::new(1.0, 1.0)
      };

      let mut offset = frame.offset - atlas.cell_origin;

      offset.x *= scale.x;
      offset.y *= scale.y;

      let src = atlas.get(frame.cell);

      let dest =
        Point2::new(position.point.x, position.point.y - position.point.z) + offset + global_offset;

      canvas
        .draw(
          &atlas.image,
          DrawParam::default().src(src).scale(scale).dest(dest),
        )
        .expect("could not draw sprite");
    }
  }

  canvas.pop_transform();
}
