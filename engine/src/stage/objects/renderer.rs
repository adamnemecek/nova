// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use ggez::graphics::DrawParam;
use std::cmp;

/// Draws the graphics for objects on the stage.
pub struct Renderer {
  /// Texture to use for shadows.
  pub shadow_texture: Arc<core::Texture>,
  /// Global scale for everything the renderer draws.
  pub scale: f32,
  /// Buffer for the queue of all entities on the stage to render during each
  /// draw call.
  ///
  /// This is used to sort entities by position before drawing.
  draw_queue: Vec<(Entity, Point3<f32>)>,
}

impl Renderer {
  /// Creates a new renderer with the default scale.
  pub fn new(shadow_texture: Arc<core::Texture>) -> Self {
    Renderer {
      shadow_texture,
      scale: 2.0,
      draw_queue: Vec::with_capacity(1024),
    }
  }

  /// Draws all the graphics on the stage.
  pub fn draw(&mut self, core: &mut Core) {
    let entities = core.world.entities();

    let viewport = core.world.read_resource::<core::Viewport>();
    let positions = core.world.read_storage::<Position>();
    let objects = core.world.read_storage::<Object>();
    let sprites = core.world.read_storage::<graphics::Sprite>();

    // Determine position of camera.
    let camera_pos = match core.world.read_resource::<Camera>().target {
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
      Point2::new(viewport.width, viewport.height) / self.scale / 2.0 - camera_pos;

    // Apply scale transform.
    ggez::graphics::push_transform(&mut core.ctx, Some(Matrix4::new_scaling(self.scale)));
    ggez::graphics::apply_transformations(&mut core.ctx).expect("could not scale for stage draw");

    // Get a reference to the shadow image.
    let shadow_image = self
      .shadow_texture
      .ggez_image
      .read()
      .expect("could not lock ggez_image");

    // Draw every object's shadow and queue its sprite to be drawn
    for (entity, object, position) in (&*entities, &objects, &positions).join() {
      if shadow_image.is_some() {
        let position = &position.point;
        let size = &object.template.shadow_size;

        ggez::graphics::draw(
          &mut core.ctx,
          shadow_image.as_ref().unwrap(),
          DrawParam::default()
            .color(ggez::graphics::Color::new(0.0, 0.0, 0.0, 0.2))
            .scale(Vector2::new(
              size.0 / self.shadow_texture.width,
              size.1 / self.shadow_texture.height,
            ))
            .dest(
              Point2::new(position.x - size.0 / 2.0, position.y - size.1 / 2.0) + global_offset,
            ),
        ).expect("could not draw sprite");
      }

      self.draw_queue.push((entity, position.point));
    }

    // Sort the queue by the y-coordinate of the position.
    self
      .draw_queue
      .sort_by(|a, b| a.1.y.partial_cmp(&b.1.y).unwrap_or(cmp::Ordering::Equal));

    // Finally, draw the sprites.
    for (entity, position) in &self.draw_queue {
      // If the entity has a sprite, draw that.
      if let Some(sprite) = sprites.get(*entity) {
        let image = sprite
          .atlas
          .texture
          .ggez_image
          .read()
          .expect("could not lock ggez_image");

        let scale = sprite.scale;
        let mut offset = sprite.offset - sprite.atlas.cell_origin;

        offset.x *= scale.x;
        offset.y *= scale.y;

        if image.is_some() {
          ggez::graphics::draw(
            &mut core.ctx,
            image.as_ref().unwrap(),
            DrawParam::default()
              .src(sprite.atlas.get(sprite.cell))
              .scale(scale)
              .dest(Point2::new(position.x, position.y - position.z) + offset + global_offset),
          ).expect("could not draw sprite");
        }
      }
    }

    ggez::graphics::pop_transform(&mut core.ctx);
    ggez::graphics::apply_transformations(&mut core.ctx)
      .expect("could not restore scale after stage draw");

    // Clear the queue for the next frame.
    self.draw_queue.clear();
  }
}
