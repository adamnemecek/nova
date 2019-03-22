// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::clock::{Duration, Instant, WriteClock};
use crate::systems::System;

#[derive(Debug)]
pub struct UpdateClock {
  pub max_delta_time: Duration,
  previous: Option<Instant>,
}

impl Default for UpdateClock {
  fn default() -> Self {
    Self {
      max_delta_time: Duration::from_hz(25),
      previous: None,
    }
  }
}

impl<'a> System<'a> for UpdateClock {
  type Data = WriteClock<'a>;

  fn run(&mut self, mut clock: Self::Data) {
    let now = Instant::now();

    let mut delta = match self.previous {
      Some(previous) => now - previous,
      None => Duration::ZERO,
    };

    if delta > self.max_delta_time {
      delta = self.max_delta_time;
    }

    clock.delta_time = delta;

    self.previous = Some(now);
  }
}
