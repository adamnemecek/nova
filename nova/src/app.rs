// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::clock;
use crate::engine::Engine;
use crate::window;
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

    clock::set_up(&mut engine);
    graphics::set_up(&mut engine).expect("Could not set up graphics");
    window::set_up(&mut engine, Default::default());
    input::set_up(&mut engine);

    App { engine }
  }

  pub fn run(mut self) {
    const MIN_FRAME_TIME: Duration = Duration::from_micros(16666); // Roughly 60 Hz.

    while !window::borrow(&self.resources).close_requested {
      let began = Instant::now();

      self.tick();

      let duration = Instant::now() - began;

      if duration < MIN_FRAME_TIME {
        spin_sleep::sleep(MIN_FRAME_TIME - duration);
      }
    }

    window::destroy(&self.resources);
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
