// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Sprite;
use crate::prelude::*;
use crate::stage::graphics::{Camera, CameraTarget};
use crate::stage::objects::Object;
use crate::stage::Position;
use nova::graphics::Canvas;
use nova::graphics::{Color, DrawParams, Image};

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
  pub shadow_image: Image,
}

/// Draws all of the objects on the stage and their shadows.
pub fn draw(ctx: &mut engine::Context, canvas: &mut Canvas, rect: &Rect<f32>) {
  let state = engine::fetch_resource::<DrawState>(ctx);
  let settings = engine::fetch_resource::<DrawSettings>(ctx);

  let positions = engine::fetch_storage::<Position>(ctx);
  let objects = engine::fetch_storage::<Object>(ctx);
  let sprites = engine::fetch_storage::<Sprite>(ctx);

  // Determine position of camera.
  let camera_pos = match engine::fetch_resource::<Camera>(ctx).target {
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
  let global_offset = rect.pos + rect.size / settings.scale / 2.0 - camera_pos;

  // Apply scale transform.
  //canvas.push_transform(Matrix4::new_scaling(settings.scale));

  // Draw object shadows.
  for entity in &state.entities {
    let position = &positions.get(*entity).unwrap().point;
    let size = &objects.get(*entity).unwrap().template.shadow_size;
    let image_size = settings.shadow_image.size();

    canvas.draw(
      &settings.shadow_image,
      DrawParams::default()
        .color(Color::new(0.0, 0.0, 0.0, 0.2))
        .scale(Vector2::new(
          size.x / image_size.x as f32,
          size.y / image_size.y as f32,
        )).dest(Point2::new(position.x - size.x / 2.0, position.y - size.y / 2.0) + global_offset),
    );
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

    let src = atlas.get(sprite.cell).into();

    let dest =
      Point2::new(position.point.x, position.point.y - position.point.z) + offset + global_offset;

    canvas.draw(
      &atlas.image,
      DrawParams::default().src(src).scale(scale).dest(dest),
    );
  }

  //canvas.pop_transform();
}
