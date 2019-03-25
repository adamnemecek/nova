// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::clock::{Clock, UpdateClock};
use crate::engine::{Engine, EnginePhase};
use crate::window::{self, WindowEvent};
use std::ops::{Deref, DerefMut};
use std::time::{Duration, Instant};

pub struct App {
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

    graphics::set_up(&mut engine.resources).expect("Could not set up graphics");
    window::set_up(&mut engine, Default::default());
    input::set_up(&mut engine);

    engine.schedule(EnginePhase::Update, UpdateClock::default());

    App { engine }
  }

  pub fn run(mut self) {
    const MIN_FRAME_TIME: Duration = Duration::from_micros(16666); // Roughly 60 Hz.

    // Register an event reader for window events.
    let mut event_reader = window::borrow_mut(&self.resources).events.register_reader();

    loop {
      let began = Instant::now();

      self.tick();

      // Exit if the player tried to close the window.
      {
        let window = window::borrow(&self.resources);
        let mut close_requested = false;

        for event in window.events.read(&mut event_reader) {
          if let WindowEvent::CloseRequested = event {
            close_requested = true;
          }
        }

        if close_requested {
          break;
        }
      }

      let duration = Instant::now() - began;

      if duration < MIN_FRAME_TIME {
        spin_sleep::sleep(MIN_FRAME_TIME - duration);
      }
    }
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
