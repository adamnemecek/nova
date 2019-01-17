// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{RealTime, Source, Time};
use crate::ecs;

#[derive(Debug)]
pub struct Ticker {
  source: Box<dyn Source>,
}

impl Ticker {
  pub fn new(source: impl Source + 'static) -> Self {
    Ticker {
      source: Box::new(source),
    }
  }
}

impl Default for Ticker {
  fn default() -> Self {
    Ticker::new(RealTime::default())
  }
}

impl<'a> ecs::System<'a> for Ticker {
  type SystemData = ecs::WriteResource<'a, Time>;

  fn setup(&mut self, res: &mut ecs::Resources) {
    res.entry().or_insert_with(Time::default);
  }

  fn run(&mut self, mut time: ecs::WriteResource<'a, Time>) {
    time.tick(self.source.delta_time());
  }
}
