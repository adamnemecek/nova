// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ggez::graphics::DrawParam;
use std::cmp;

use prelude::*;

use super::Position;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Render;

#[derive(Default)]
pub struct Renderer {
  draw_queue: Vec<(Entity, Position)>,
}

impl Renderer {
  pub fn draw(&mut self, core: &mut Core) {
    // Add all entities with the `Render` and `Position` components to the
    // queue.
    let entities = core.world.entities();
    let rendered = core.world.read_storage::<Render>();
    let positions = core.world.read_storage::<Position>();

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
        let x = position.x.round();
        let y = (position.y - position.z).round();

        ggez::graphics::draw(
          &mut core.ctx,
          &sprite.atlas.image,
          DrawParam::default()
            .src(sprite.atlas.cells[sprite.cell])
            .dest(Point2::new(x, y)),
        ).expect("could not draw sprite");
      }
    }

    // Clear the queue for the next frame.
    self.draw_queue.clear();
  }
}
