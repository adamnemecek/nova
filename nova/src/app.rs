// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{assets, graphics};
use crate::clock::{Clock, UpdateClock};
use crate::engine::{Engine, EnginePhase};
use crate::renderer::Renderer;
use crate::ui;
use crate::window::{self, Window};
use std::ops::{Deref, DerefMut};
use std::time::{Duration, Instant};

pub struct App {
  ui_painter: ui::Painter,
  renderer: Renderer,
  pub engine: Engine,
}

impl Default for App {
  fn default() -> Self {
    App::new()
  }
}

impl App {
  pub fn new() -> Self {
    let mut engine = Engine::new();

    engine.resources.insert(Clock::default());

    assets::setup(&mut engine, Default::default());
    graphics::setup(&mut engine);
    Window::setup(&mut engine, Default::default());
    input::setup(&mut engine);
    ui::setup(&mut engine);

    engine.schedule(EnginePhase::BeforeUpdate, UpdateClock::default());

    let mut renderer = Renderer::new(&engine);
    let ui_painter = ui::Painter::new(&mut renderer);

    App {
      ui_painter,
      renderer,
      engine,
    }
  }

  pub fn run(mut self) {
    const MIN_FRAME_TIME: Duration = Duration::from_micros(16666); // Roughly 60 Hz.

    // Register an event reader for window events.
    let mut event_reader = {
      let mut events = self.resources.fetch_mut::<window::Events>();

      events.channel_mut().register_reader()
    };

    loop {
      let began = Instant::now();

      self.tick();

      // Exit if the player tried to close the window.
      {
        let events = self.resources.fetch::<window::Events>();
        let mut close_requested = false;

        for event in events.channel().read(&mut event_reader) {
          if let window::Event::CloseRequested = event {
            close_requested = true;
          }
        }

        if close_requested {
          break;
        }
      }

      self.render();

      let duration = Instant::now() - began;

      if duration < MIN_FRAME_TIME {
        spin_sleep::sleep(MIN_FRAME_TIME - duration);
      }
    }

    self.destroy();
  }

  pub fn tick(&mut self) {
    self.engine.tick();

    ui::messages::deliver(&self.resources);
    ui::nodes::build(&self.resources);
  }

  pub fn render(&mut self) {
    let mut render = self.renderer.begin();

    self.ui_painter.draw(&mut render, &self.engine.resources);

    self.renderer.finish(&self.engine.resources);
  }

  pub fn destroy(self) {
    self.ui_painter.destroy(self.renderer.device());
    self.renderer.destroy();
  }
}

impl Deref for App {
  type Target = Engine;

  fn deref(&self) -> &Engine {
    &self.engine
  }
}

impl DerefMut for App {
  fn deref_mut(&mut self) -> &mut Engine {
    &mut self.engine
  }
}
