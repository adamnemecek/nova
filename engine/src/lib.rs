// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate ggez;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate specs;

pub mod core;
pub mod rendering;
pub mod sprites;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::winit_event::{Event, WindowEvent};
use specs::prelude::*;
use std::env;
use std::error::Error;
use std::path::PathBuf;

pub struct Engine<'a, 'b> {
  pub world: World,
  pub ctx: ggez::Context,
  events_loop: ggez::event::EventsLoop,
  dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Engine<'a, 'b> {
  pub fn new() -> Result<Self, Box<dyn Error>> {
    let mut world = World::new();

    world.register::<core::Position>();
    world.register::<rendering::Rendered>();
    world.register::<sprites::Sprite>();

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

      builder.build()?
    };

    let dispatcher = {
      let builder = DispatcherBuilder::new();

      builder.build()
    };

    Ok(Engine {
      world,
      ctx,
      events_loop,
      dispatcher,
    })
  }

  pub fn run(mut self) -> Result<(), Box<dyn Error>> {
    let mut renderer = rendering::Renderer::new();

    while self.ctx.continuing {
      self.ctx.timer_context.tick();

      {
        let ctx = &mut self.ctx;

        self.events_loop.poll_events(|event| match event {
          Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
              ctx.quit();
            }

            WindowEvent::Resized(size) => {
              ggez::graphics::set_screen_coordinates(
                ctx,
                ggez::graphics::Rect::new(0.0, 0.0, size.width as f32, size.height as f32),
              ).unwrap();
            }

            _ => (),
          },

          _ => (),
        });
      }

      self.dispatcher.dispatch(&mut self.world.res);

      renderer.draw(&mut self)?;
    }

    Ok(())
  }
}
