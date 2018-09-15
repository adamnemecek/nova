// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ggez::graphics::DrawParam;
use specs::prelude::*;
use std::cmp;

use prelude::*;

pub mod drawable;
pub mod position;
pub mod sprite;

pub use self::drawable::Drawable;
pub use self::position::Position;
pub use self::sprite::Sprite;

pub struct Stage {
  draw_queue: Vec<(Entity, Position)>,
}

impl Stage {
  pub fn new(core: &mut Core) -> Stage {
    core.world.register::<Drawable>();
    core.world.register::<Position>();
    core.world.register::<Sprite>();

    Stage {
      draw_queue: Vec::with_capacity(1024),
    }
  }

  pub fn draw(&mut self, core: &mut Core) {
    // Add all entities with the `Drawable` and `Position` components to the
    // queue.
    let entities = core.world.entities();
    let drawables = core.world.read_storage::<Drawable>();
    let positions = core.world.read_storage::<Position>();

    for (entity, _, position) in (&*entities, &drawables, &positions).join() {
      self.draw_queue.push((entity, *position));
    }

    // Sort the queue by the y-coordinate of the position.
    self
      .draw_queue
      .sort_by(|a, b| a.1.y.partial_cmp(&b.1.y).unwrap_or(cmp::Ordering::Equal));

    // Finally, draw the entities.
    let sprites = core.world.read_storage::<Sprite>();

    ggez::graphics::clear(&mut core.ctx, ggez::graphics::BLACK);

    for (entity, position) in &self.draw_queue {
      // If the entity has a sprite, draw that.
      if let Some(sprite) = sprites.get(*entity) {
        ggez::graphics::draw(
          &mut core.ctx,
          &sprite.atlas.image,
          DrawParam::default()
            .src(sprite.atlas.cells[sprite.cell])
            .dest(ggez::nalgebra::Point2::new(
              position.x,
              position.y - position.z,
            )),
        ).expect("could not draw sprite");
      }
    }

    // Clear the queue for the next frame.
    self.draw_queue.clear();
  }
}
