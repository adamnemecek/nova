extern crate ggez;
extern crate specs;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::graphics::{DrawParam, Rect, Text};

struct Game<'a, 'b> {
  dispatcher: specs::Dispatcher<'a, 'b>,
  world: specs::World,
  fps_text: Text,
}

impl<'a, 'b> Game<'a, 'b> {
  fn new() -> ggez::GameResult<Game<'a, 'b>> {
    let dispatcher = specs::DispatcherBuilder::new().build();
    let world = specs::World::new();

    Ok(Game {
      dispatcher,
      world,
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

    // Draw the FPS counter in the top left of the screen.
    ggez::graphics::draw(ctx, &mut self.fps_text, DrawParam::default())?;

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
  let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("nova", "bfrydl")
    // Create a resizable window with vsync disabled.
    .window_mode(WindowMode::default().resizable(true))
    .window_setup(WindowSetup::default().title("nova").vsync(false))
    .build()?;

  // Create a new game instance.
  let mut game = Game::new()?;

  // Run the main event loop.
  ggez::event::run(&mut ctx, &mut event_loop, &mut game)?;

  Ok(())
}
