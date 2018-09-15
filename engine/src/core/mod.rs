// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ggez::event::winit_event::*;

use prelude::*;

pub mod clock;
pub mod context;
pub mod fps;
pub mod keyboard;

pub use self::clock::Clock;

pub type Tick = u64;

pub struct Core {
  pub world: World,
  pub(crate) ctx: ggez::Context,
  events_loop: ggez::event::EventsLoop,
}

impl Core {
  pub fn new(ctx_builder: ggez::ContextBuilder) -> Self {
    let (ctx, events_loop) = ctx_builder.build().expect("could not create ggez::Context");
    let mut world = World::new();

    world.add_resource(Clock::default());
    world.add_resource(keyboard::Events::default());

    Core {
      world,
      ctx,
      events_loop,
    }
  }

  pub fn is_running(&self) -> bool {
    self.ctx.continuing
  }

  pub fn update(&mut self) {
    let ctx = &mut self.ctx;

    // Present the previous frame and clear the buffer.
    ggez::graphics::present(ctx).expect("could not present");
    ggez::graphics::clear(ctx, ggez::graphics::BLACK);

    // Progress time.
    let mut clock = self.world.write_resource::<Clock>();

    ctx.timer_context.tick();

    clock.tick += 1;
    clock.delta_time = ggez::timer::duration_to_f64(ggez::timer::delta(ctx));
    clock.time += clock.delta_time;
    clock.fps = ggez::timer::fps(ctx);

    // Process events.
    let mut kb_events = self.world.write_resource::<keyboard::Events>();

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
