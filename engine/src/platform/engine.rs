// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::winit_event::*;
use ggez::event::EventsLoop;
use std::env;
use std::path::PathBuf;

use prelude::*;

use super::Keyboard;

pub struct Engine {
  pub ctx: ggez::Context,
  events_loop: EventsLoop,
}

impl Engine {
  pub fn new(world: &mut World) -> Engine {
    world.add_resource(Keyboard::default());

    // Create a new ggez context and winit events loop.
    let (ctx, events_loop) = {
      let mut builder = ggez::ContextBuilder::new("nova", "bfrydl")
        // Create a resizable window with vsync disabled.
        .window_mode(WindowMode::default().resizable(true))
        .window_setup(WindowSetup::default().title("nova").vsync(false));

      // Add the resources dir for development.
      if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);

        path.push("resources");
        builder = builder.add_resource_path(path);
      }

      builder.build().expect("could not create platform context")
    };

    Engine { ctx, events_loop }
  }

  pub fn update(&mut self, world: &World) {
    let ctx = &mut self.ctx;

    let time = world.read_resource::<time::Clock>().time;
    let mut keyboard = world.write_resource::<Keyboard>();

    self.events_loop.poll_events(|event| match event {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::KeyboardInput { input, .. } => {
          if let Some(key) = input.virtual_keycode {
            match input.state {
              ElementState::Pressed => keyboard.set_pressed(key, time),
              ElementState::Released => keyboard.set_released(key),
            }
          }
        }

        WindowEvent::CloseRequested => {
          ctx.quit();
        }

        WindowEvent::Resized(size) => {
          ggez::graphics::set_screen_coordinates(
            ctx,
            ggez::graphics::Rect::new(0.0, 0.0, size.width as f32 / 2.0, size.height as f32 / 2.0),
          ).unwrap();
        }

        _ => {}
      },

      _ => {}
    });
  }
}
