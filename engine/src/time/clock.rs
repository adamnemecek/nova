// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Duration, Instant, Settings};
use crate::Engine;

pub struct Clock {
  pub updated_at: Instant,
  pub time: Duration,
  pub delta_time: Duration,
}

impl Clock {
  pub fn new() -> Self {
    Clock::default()
  }

  pub fn update(&mut self, settings: &Settings) {
    let now = Instant::now();

    let delta_time = match now - self.updated_at {
      x if x > settings.max_delta_time => settings.max_delta_time,
      x if x < settings.min_delta_time => settings.min_delta_time,
      x => x,
    };

    self.updated_at = now;
    self.delta_time = delta_time;
    self.time += delta_time;
  }
}

impl Default for Clock {
  fn default() -> Self {
    Clock {
      updated_at: Instant::now(),
      time: Duration::ZERO,
      delta_time: Duration::ZERO,
    }
  }
}

pub fn update_clock(engine: &mut Engine) {
  let settings = engine.fetch_resource::<Settings>();
  let mut clock = engine.fetch_resource_mut::<Clock>();

  clock.update(&settings);
}
