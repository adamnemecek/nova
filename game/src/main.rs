extern crate ggez;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate specs;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::graphics::{DrawParam, Rect, Text};
use ggez::nalgebra::Point2;
use specs::prelude::*;
use std::sync::Arc;

mod core;
mod sprites;

struct Game<'a, 'b> {
  world: specs::World,
  dispatcher: specs::Dispatcher<'a, 'b>,
  fps_text: Text,
}

impl<'a, 'b> Game<'a, 'b> {
  fn new() -> ggez::GameResult<Game<'a, 'b>> {
    let mut world = specs::World::new();

    world.register::<core::Position>();
    world.register::<sprites::Sprite>();

    let dispatcher = specs::DispatcherBuilder::new().build();

    Ok(Game {
      world,
      dispatcher,
      fps_text: Text::new("FPS: ?"),
    })
  }
}

impl<'a, 'b> ggez::event::EventHandler for Game<'a, 'b> {
  /// Invoked on each frame to update the game.
  fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
    // Every 60th of a second, update the `TextCached` with current FPS.
    if ggez::timer::check_update_time(ctx, 60) {
      let fragment = &mut self.fps_text.fragments_mut()[0];

      fragment.text = format!("FPS: {}", ggez::timer::fps(ctx) as u32);
    }

    self.dispatcher.dispatch(&mut self.world.res);

    Ok(())
  }

  /// Invoked on each frame to draw the game.
  fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
    ggez::graphics::clear(ctx, ggez::graphics::BLACK);

    // Draw all sprites with positions.
    let sprites = self.world.read_storage::<sprites::Sprite>();
    let positions = self.world.read_storage::<core::Position>();

    let mut data = (&sprites, &positions).join().collect::<Vec<_>>();

    data.sort_by(|(_, a), (_, b)| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal));

    for (sprite, position) in data {
      ggez::graphics::draw(
        ctx,
        &sprite.atlas.image,
        DrawParam::default()
          .src(sprite.atlas.frames[sprite.frame])
          .dest(Point2::new(position.x, position.y - position.z)),
      )?;
    }

    // Draw the FPS counter in the top left of the screen.
    ggez::graphics::draw(ctx, &self.fps_text, DrawParam::default())?;

    ggez::graphics::present(ctx)?;

    Ok(())
  }

  /// Invoked when the window is resized.
  fn resize_event(&mut self, ctx: &mut ggez::Context, width: f32, height: f32) {
    // Change the screen coordinates to match the window size.
    ggez::graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height)).unwrap();
  }
}

/// Main entry point of the program.
pub fn main() -> Result<(), Box<std::error::Error>> {
  // Create a new ggez Context.
  let (mut ctx, mut event_loop) = {
    let mut builder = ggez::ContextBuilder::new("nova", "bfrydl")
    // Create a resizable window with vsync disabled.
    .window_mode(WindowMode::default().resizable(true))
    .window_setup(WindowSetup::default().title("nova").vsync(false));

    // Add the resources dir for development.
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
      let mut path = std::path::PathBuf::from(manifest_dir);

      path.push("resources");
      builder = builder.add_resource_path(path);
    }

    builder.build()?
  };

  // Create a new game instance.
  let mut game = Game::new()?;

  game
    .world
    .create_entity()
    .with(core::Position {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    })
    .with(sprites::Sprite {
      atlas: Arc::new(sprites::Atlas::new(&mut ctx, "/charizard")?),
      frame: 0,
    })
    .build();

  game
    .world
    .create_entity()
    .with(core::Position {
      x: 0.0,
      y: -10.0,
      z: 0.0,
    })
    .with(sprites::Sprite {
      atlas: Arc::new(sprites::Atlas::new(&mut ctx, "/venusaur")?),
      frame: 0,
    })
    .build();

  // Run the main event loop.
  ggez::event::run(&mut ctx, &mut event_loop, &mut game)?;

  Ok(())
}
