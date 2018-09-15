// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ggez;
use ggez::graphics::{DrawParam, Text, TextFragment};
use specs::prelude::*;
use std::cmp;
use std::error::Error;

use prelude::*;

use super::{Drawable, Sprite};

/// Engine that draws the game's graphics.
pub struct Engine {
  /// Queue of entries that is filled each frame for sorting draw calls.
  queue: Vec<QueueEntry>,
  /// Text for displaying the current FPS.
  fps_text: Text,
}

impl Engine {
  /// Initializes a new engine for the given `world`.
  pub fn new(world: &mut World) -> Engine {
    world.register::<Sprite>();
    world.register::<Drawable>();

    Engine {
      queue: Vec::with_capacity(1024),
      fps_text: Text::default(),
    }
  }

  /// Draws one frame for the given `world` on the given `platform`.
  pub fn draw(
    &mut self,
    world: &World,
    platform: &mut platform::Engine,
  ) -> Result<(), Box<dyn Error>> {
    // Add all entities with the `Drawable` and `Position` components to the
    // queue.
    let entities = world.entities();
    let drawables = world.read_storage::<Drawable>();
    let positions = world.read_storage::<motion::Position>();

    for (entity, _, position) in (&*entities, &drawables, &positions).join() {
      self.queue.push(QueueEntry {
        entity,
        position: *position,
      });
    }

    // Sort the queue by the y-coordinate of the position.
    self.queue.sort_by(|a, b| {
      a.position
        .y
        .partial_cmp(&b.position.y)
        .unwrap_or(cmp::Ordering::Equal)
    });

    // Finally, draw the entities.
    let sprites = world.read_storage::<Sprite>();

    ggez::graphics::clear(&mut platform.ctx, ggez::graphics::BLACK);

    for entry in &self.queue {
      // If the entity has a sprite, draw that.
      if let Some(sprite) = sprites.get(entry.entity) {
        ggez::graphics::draw(
          &mut platform.ctx,
          &sprite.atlas.image,
          DrawParam::default()
            .src(sprite.atlas.cells[sprite.cell])
            .dest(ggez::nalgebra::Point2::new(
              entry.position.x,
              entry.position.y - entry.position.z,
            )),
        )?;
      }
    }

    // Draw the current FPS on the screen.
    ggez::graphics::draw(&mut platform.ctx, &self.fps_text, DrawParam::default())?;

    ggez::graphics::present(&mut platform.ctx)?;

    // Clear the queue for the next frame.
    self.queue.clear();

    // Update the current FPS once a second.
    if ggez::timer::check_update_time(&mut platform.ctx, 1) {
      self.fps_text = Text::new(TextFragment::from(format!(
        "FPS: {}",
        ggez::timer::fps(&mut platform.ctx) as u32
      )));
    }

    Ok(())
  }
}

/// An entry in the draw queue for a particular frame.
struct QueueEntry {
  /// The entity to draw.
  entity: Entity,
  /// The position of the entity.
  position: motion::Position,
}
