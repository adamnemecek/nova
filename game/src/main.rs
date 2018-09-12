extern crate ggez;
extern crate specs;

use ggez::graphics::{Point2, Rect, TextCached, TextFragment};

struct Game<'a, 'b> {
  dispatcher: specs::Dispatcher<'a, 'b>,
  world: specs::World,
  fps_text: TextCached,
}

impl<'a, 'b> Game<'a, 'b> {
  fn new() -> ggez::GameResult<Game<'a, 'b>> {
    let dispatcher = specs::DispatcherBuilder::new().build();
    let world = specs::World::new();

    Ok(Game {
      dispatcher,
      world,
      fps_text: TextCached::new("FPS: ?")?,
    })
  }
}

impl<'a, 'b> ggez::event::EventHandler for Game<'a, 'b> {
  /// Invoked on each frame to update the game.
  fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
    // Every 60th of a second, update the `TextCached` with current FPS.
    if ggez::timer::check_update_time(ctx, 60) {
      self.fps_text.replace_fragment(
        0,
        TextFragment::from(format!("FPS: {}", ggez::timer::get_fps(ctx) as u32)),
      );
    }

    self.dispatcher.dispatch(&mut self.world.res);

    Ok(())
  }

  /// Invoked on each frame to draw the game.
  fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
    ggez::graphics::clear(ctx);

    // Draw the FPS counter in the top left of the screen.
    ggez::graphics::draw(ctx, &mut self.fps_text, Point2::new(0.0, 0.0), 0.0)?;

    ggez::graphics::present(ctx);

    Ok(())
  }

  /// Invoked when the window is resized.
  fn resize_event(&mut self, ctx: &mut ggez::Context, width: u32, height: u32) {
    // Change the screen coordinates to match the window size.
    ggez::graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width as f32, height as f32))
      .unwrap();
  }
}

/// Main entry point of the program.
pub fn main() -> Result<(), Box<std::error::Error>> {
  // Create a new ggez Context.
  let mut ctx = ggez::ContextBuilder::new("nova", "bfrydl")
    .window_mode(ggez::conf::WindowMode::default().vsync(false))
    // Create a resizable window.
    .window_setup(
      ggez::conf::WindowSetup::default()
        .title("nova")
        .resizable(true),
    )
    .build()?;

  // Create a new game instance.
  let mut game = Game::new()?;

  // Run the main event loop.
  ggez::event::run(&mut ctx, &mut game)?;

  Ok(())
}
