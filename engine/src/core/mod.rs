// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ggez::event::winit_event::*;

use prelude::*;

pub mod clock;
pub mod context;
pub mod fps_display;
pub mod fs;
pub mod keyboard;
pub mod viewport;

pub use self::clock::Clock;
pub use self::fps_display::FpsDisplay;
pub use self::viewport::Viewport;

/// Number of ticks (game loops) since launch.
pub type Tick = u64;

/// Provides core engine functionality.
pub struct Core {
  /// ECS world state.
  pub world: World,
  /// ggez context.
  pub(crate) ctx: ggez::Context,
  /// winit events loop.
  events_loop: ggez::event::EventsLoop,
}

impl Core {
  /// Creates a new core from the given ggez context builder.
  pub fn new(ctx_builder: ggez::ContextBuilder) -> Self {
    let (mut ctx, events_loop) = ctx_builder.build().expect("could not create ggez::Context");
    let mut world = World::new();

    world.add_resource(Clock::default());
    world.add_resource(keyboard::Events::default());
    world.add_resource(Viewport::from(ggez::graphics::screen_coordinates(&mut ctx)));

    Core {
      world,
      ctx,
      events_loop,
    }
  }

  /// Returns `true` until the game loop should quit.
  pub fn is_running(&self) -> bool {
    self.ctx.continuing
  }

  /// Updates the core, running through one engine tick.
  pub fn tick(&mut self) {
    let ctx = &mut self.ctx;
    let world = &mut self.world;

    // Present the previous frame and clear the buffer.
    ggez::graphics::present(ctx).expect("could not present");
    ggez::graphics::clear(ctx, ggez::graphics::BLACK);

    // Progress time.
    let mut clock = world.write_resource::<Clock>();

    ctx.timer_context.tick();

    clock.tick += 1;
    clock.delta_time = ggez::timer::duration_to_f64(ggez::timer::delta(ctx));
    clock.time += clock.delta_time;
    clock.fps = ggez::timer::fps(ctx);

    // Process events.
    let mut kb_events = world.write_resource::<keyboard::Events>();

    kb_events.list.clear();

    self.events_loop.poll_events(|event| match event {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::CloseRequested => {
          ctx.quit();
        }

        WindowEvent::Resized(size) => {
          ggez::graphics::set_screen_coordinates(
            ctx,
            ggez::graphics::Rect::new(0.0, 0.0, size.width as f32, size.height as f32),
          ).expect("could not resize");

          let mut viewport = world.write_resource::<Viewport>();

          viewport.width = size.width as f32;
          viewport.height = size.height as f32;
        }

        WindowEvent::KeyboardInput { input, .. } => {
          if let Some(key) = input.virtual_keycode {
            kb_events.list.push(match input.state {
              ElementState::Pressed => keyboard::Event::Pressed(key),
              ElementState::Released => keyboard::Event::Released(key),
            });
          }
        }

        _ => {}
      },

      _ => {}
    });
  }
}
