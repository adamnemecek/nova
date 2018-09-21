// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use graphics::DrawParams;

/// State of object drawing.
#[derive(Default)]
pub struct DrawState {
  /// Entities on the stage that have an `Object` component, in order from
  /// background to foreground.
  pub entities: Vec<Entity>,
}

/// Settings for object drawing.
pub struct DrawSettings {
  /// Global scale for drawing objects.
  pub scale: f32,
  /// Image to use for drawing shadows.
  pub shadow_image: graphics::Image,
}

/// Draws all the objects on the stage.
pub fn draw(world: &mut World, canvas: &mut graphics::Canvas) {
  let state = world.read_resource::<DrawState>();
  let settings = world.read_resource::<DrawSettings>();

  let positions = world.read_storage::<Position>();
  let objects = world.read_storage::<Object>();
  let sprites = world.read_storage::<Sprite>();

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
  let global_offset = Point2::from_coordinates(canvas.size() / settings.scale / 2.0) - camera_pos;

  // Apply scale transform.
  canvas.push_transform(Matrix4::new_scaling(settings.scale));

  // Draw object shadows.
  for entity in &state.entities {
    let position = &positions.get(*entity).unwrap().point;
    let size = &objects.get(*entity).unwrap().template.shadow_size;
    let image_size = settings.shadow_image.size();

    canvas
      .draw(
        &settings.shadow_image,
        DrawParams::default()
          .color(ggez::graphics::Color::new(0.0, 0.0, 0.0, 0.2))
          .scale(Vector2::new(
            size.x / image_size.x as f32,
            size.y / image_size.y as f32,
          ))
          .dest(Point2::new(position.x - size.x / 2.0, position.y - size.y / 2.0) + global_offset),
      )
      .expect("could not draw sprite");
  }

  // Draw object sprites.
  for entity in &state.entities {
    let position = positions.get(*entity).unwrap();
    let object = objects.get(*entity).unwrap();
    let sprite = sprites.get(*entity).unwrap();

    let atlas = &object.template.atlas;

    let scale = if sprite.hflip {
      Vector2::new(-1.0, 1.0)
    } else {
      Vector2::new(1.0, 1.0)
    };

    let mut offset = sprite.offset - atlas.cell_origin;

    offset.x *= scale.x;
    offset.y *= scale.y;

    let src = atlas.get(sprite.cell);

    let dest =
      Point2::new(position.point.x, position.point.y - position.point.z) + offset + global_offset;

    canvas
      .draw(
        &atlas.image,
        DrawParams::default().src(src).scale(scale).dest(dest),
      )
      .expect("could not draw sprite");
  }

  canvas.pop_transform();
}