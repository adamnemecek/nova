use ggez;
use specs::prelude::*;
use std::cmp;
use std::error::Error;

use {core, sprites, Engine};

/// A component that flags an entity to be drawn by the `Renderer`.
#[derive(Debug, Default)]
pub struct Rendered;

impl Component for Rendered {
  type Storage = NullStorage<Self>;
}

/// Renders entities with the `Rendered` component.
pub struct Renderer {
  /// Queue of entries that is filled each frame for sorting draw calls.
  queue: Vec<QueueEntry>,
}

/// An entry in the draw queue for a particular frame.
struct QueueEntry {
  /// The entity to draw.
  entity: Entity,
  /// The position of the entity.
  position: core::Position,
}

impl Renderer {
  /// Initializes a new renderer.
  pub fn new() -> Renderer {
    Renderer {
      queue: Vec::with_capacity(1024),
    }
  }

  /// Draws one frame for the given `engine`.
  pub fn draw(&mut self, engine: &mut Engine) -> Result<(), Box<dyn Error>> {
    // Add all entities with the `Rendered` and `Position` components to the
    // queue.
    let entities = engine.world.entities();
    let rendereds = engine.world.read_storage::<Rendered>();
    let positions = engine.world.read_storage::<core::Position>();

    for (entity, _, position) in (&*entities, &rendereds, &positions).join() {
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
    let sprites = engine.world.read_storage::<sprites::Sprite>();

    ggez::graphics::clear(&mut engine.ctx, ggez::graphics::BLACK);

    for entry in &self.queue {
      // If the entity has a sprite, draw that.
      if let Some(sprite) = sprites.get(entry.entity) {
        ggez::graphics::draw(
          &mut engine.ctx,
          &sprite.atlas.image,
          ggez::graphics::DrawParam::default()
            .src(sprite.atlas.frames[sprite.frame])
            .dest(ggez::nalgebra::Point2::new(
              entry.position.x,
              entry.position.y - entry.position.z,
            )),
        )?;
      }
    }

    ggez::graphics::present(&mut engine.ctx)?;

    // Clear the queue for the next frame.
    self.queue.clear();

    Ok(())
  }
}
