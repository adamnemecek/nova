// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use ggez::event::winit_event::*;

pub mod input;

mod assets;
mod texture;
mod viewport;

pub use self::assets::*;
pub use self::texture::*;
pub use self::viewport::*;

/// Provides core engine functionality.
pub struct Core {
  /// Application name, used in the window title and in directory names.
  pub app_name: &'static str,
  /// ECS world.
  pub world: World,
  /// Low-level engine context.
  pub(crate) ctx: ggez::Context,
  /// Low-level window events loop.
  events_loop: ggez::event::EventsLoop,
}

impl Core {
  /// Creates a new `Core` and initializes the engine.
  ///
  /// Application name is used in the window title, while application name and
  /// author are used in directory names.
  pub fn new(app_name: &'static str, author: &'static str) -> Self {
    let (mut ctx, events_loop) = ggez::ContextBuilder::new(app_name, author)
      .window_mode(ggez::conf::WindowMode::default().resizable(true))
      .window_setup(
        ggez::conf::WindowSetup::default()
          .title(app_name)
          .vsync(false),
      )
      .build()
      .expect("could not create ggez::Context");

    // Initialize ECS world with core resources.
    let mut world = World::new();

    world.add_resource(Assets::default());
    world.add_resource(Viewport::from(ggez::graphics::screen_coordinates(&mut ctx)));
    world.add_resource(input::KeyEvents::default());

    Core {
      app_name,
      world,
      ctx,
      events_loop,
    }
  }

  /// Returns `true` until the game loop should quit.
  pub fn is_running(&self) -> bool {
    self.ctx.continuing
  }

  /// Updates core engine resources.
  ///
  /// This method should be called once per game loop.
  pub fn tick(&mut self) {
    let ctx = &mut self.ctx;
    let world = &mut self.world;

    // Present the previous frame and clear the buffer for the next one.
    //
    // This may seem backwards doing this first but in a game loop it works out fine.
    ggez::graphics::present(ctx).expect("could not present");
    ggez::graphics::clear(ctx, ggez::graphics::Color::new(0.53, 0.87, 0.52, 1.0));

    // Show FPS in the window title.
    ctx.timer_context.tick();

    ggez::graphics::set_window_title(
      ctx,
      &format!("{} ({} FPS)", self.app_name, ggez::timer::fps(ctx) as usize),
    );

    // Load queued resources for assets (e.g. images).
    world.read_resource::<Assets>().load_queued_resources(ctx);

    // Clear the `KeyEvents` resource of events from the previous tick.
    let mut key_events = world.write_resource::<input::KeyEvents>();

    key_events.list.clear();

    // Process window events and update the associated resources.
    self.events_loop.poll_events(|event| {
      let event = ctx.process_event(&event);

      match event {
        Event::WindowEvent { event, .. } => match event {
          // When the window is closing, quit the game loop.
          WindowEvent::CloseRequested => {
            ctx.quit();
          }

          // When the window is resized, adjust screen size to match.
          WindowEvent::Resized(size) => {
            ggez::graphics::set_screen_coordinates(
              ctx,
              ggez::graphics::Rect::new(0.0, 0.0, size.width as f32, size.height as f32),
            ).expect("could not resize");

            // Update the `Viewport` resource with the current screen size.
            let mut viewport = world.write_resource::<Viewport>();

            viewport.width = size.width as f32;
            viewport.height = size.height as f32;
          }

          // When a key is pressed or released, add an event to the `KeyEvents`
          // resource.
          WindowEvent::KeyboardInput { input, .. } => {
            if let Some(key) = input.virtual_keycode {
              key_events.list.push(match input.state {
                ElementState::Pressed => input::KeyEvent::Pressed(key),
                ElementState::Released => input::KeyEvent::Released(key),
              });
            }
          }

          _ => {}
        },

        _ => {}
      };
    });
  }
}
