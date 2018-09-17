// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ggez::graphics::DrawParam;
use std::cmp;

use prelude::*;

use super::{camera, Camera, Position};

/// Component that indicates that an entity should be drawn by a `Renderer`.
#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Render;

/// Renders the stage onto the screen.
pub struct Renderer {
  /// Render scale multiplier.
  pub scale: f32,
  /// A queue of entities to draw this frame for sorting by position.
  draw_queue: Vec<(Entity, Point3<f32>)>,
}

impl Renderer {
  /// Draws the stage.
  pub fn draw(&mut self, core: &mut Core) {
    let entities = core.world.entities();

    let viewport = core.world.read_resource::<core::Viewport>();
    let camera = core.world.read_resource::<Camera>();

    let rendered = core.world.read_storage::<Render>();
    let positions = core.world.read_storage::<Position>();

    // Determine position of camera.
    let camera_pos = match camera.target {
      camera::Target::Position(pos) => pos,
      camera::Target::Entity(entity) => {
        if let Some(pos) = positions.get(entity) {
          Point2::new(pos.point.x, pos.point.y)
        } else {
          Point2::new(0.0, 0.0)
        }
      }
    };

    // Calculate the offset in drawing needed for the camera's position.
    let draw_offset = Point2::new(viewport.width, viewport.height) / self.scale / 2.0 - camera_pos;

    // Queue all rendered entities for drawing.
    for (entity, _, position) in (&*entities, &rendered, &positions).join() {
      self.draw_queue.push((entity, position.point));
    }

    // Sort the queue by the y-coordinate of the position.
    self
      .draw_queue
      .sort_by(|a, b| a.1.y.partial_cmp(&b.1.y).unwrap_or(cmp::Ordering::Equal));

    // Finally, draw the entities.
    let sprites = core.world.read_storage::<graphics::Sprite>();

    ggez::graphics::push_transform(&mut core.ctx, Some(Matrix4::new_scaling(self.scale)));
    ggez::graphics::apply_transformations(&mut core.ctx).expect("could not scale for stage draw");

    for (entity, position) in &self.draw_queue {
      // If the entity has a sprite, draw that.
      if let Some(sprite) = sprites.get(*entity) {
        let mut param = DrawParam::default();

        param = param.src(sprite.atlas.get(sprite.cell));
        param = param.dest(Point2::new(position.x, position.y - position.z) + draw_offset);

        if sprite.hflip {
          param = param.scale(Vector2::new(-1.0, 1.0));
        }

        ggez::graphics::draw(&mut core.ctx, &sprite.atlas.image, param)
          .expect("could not draw sprite");
      }
    }

    ggez::graphics::pop_transform(&mut core.ctx);
    ggez::graphics::apply_transformations(&mut core.ctx)
      .expect("could not restore scale after stage draw");

    // Clear the queue for the next frame.
    self.draw_queue.clear();
  }
}

impl Default for Renderer {
  fn default() -> Self {
    Renderer {
      scale: 2.0,
      draw_queue: Vec::with_capacity(1024),
    }
  }
}
