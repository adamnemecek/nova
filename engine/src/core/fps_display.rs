// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ggez::graphics::{DrawParam, Text, TextFragment};

use prelude::*;

use super::Clock;

#[derive(Default)]
pub struct FpsDisplay {
  elapsed: f64,
  text: Text,
}

impl FpsDisplay {
  pub fn update(&mut self, core: &Core) {
    let clock = core.world.read_resource::<Clock>();

    self.elapsed += clock.delta_time;

    if self.elapsed >= 1.0 {
      self.text = Text::new(TextFragment::from(format!("FPS: {}", clock.fps as u32)));
      self.elapsed = 0.0;
    }
  }

  pub fn draw(&self, core: &mut Core) {
    ggez::graphics::draw(&mut core.ctx, &self.text, DrawParam::default())
      .expect("could not draw fps counter");
  }
}
