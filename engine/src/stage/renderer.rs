// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ggez::graphics::DrawParam;
use std::cmp;

use prelude::*;

use super::{camera, Camera, Position};

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Render;

#[derive(Default)]
pub struct Renderer {
  draw_queue: Vec<(Entity, Position)>,
}

impl Renderer {
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
          Point2::new(pos.x, pos.y)
        } else {
          Point2::new(0.0, 0.0)
        }
      }
    };

    let draw_offset = Point2::new(viewport.width / 2.0, viewport.height / 2.0) - camera_pos;

    // Queue all rendered entities for drawing.
    for (entity, _, position) in (&*entities, &rendered, &positions).join() {
      self.draw_queue.push((entity, *position));
    }

    // Sort the queue by the y-coordinate of the position.
    self
      .draw_queue
      .sort_by(|a, b| a.1.y.partial_cmp(&b.1.y).unwrap_or(cmp::Ordering::Equal));

    // Finally, draw the entities.
    let sprites = core.world.read_storage::<graphics::Sprite>();

    ggez::graphics::clear(&mut core.ctx, ggez::graphics::BLACK);

    for (entity, position) in &self.draw_queue {
      // If the entity has a sprite, draw that.
      if let Some(sprite) = sprites.get(*entity) {
        let src = sprite.atlas.cells[sprite.cell];
        let mut dest = Point2::new(position.x, position.y - position.z);

        dest += draw_offset;

        ggez::graphics::draw(
          &mut core.ctx,
          &sprite.atlas.image,
          DrawParam::default().src(src).dest(dest),
        ).expect("could not draw sprite");
      }
    }

    // Clear the queue for the next frame.
    self.draw_queue.clear();
  }
}
