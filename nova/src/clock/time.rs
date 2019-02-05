// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Duration, Instant};
use crate::ecs;

#[derive(Debug)]
pub struct Time {
  pub delta: Duration,
  pub max_delta: Duration,
}

impl Default for Time {
  fn default() -> Self {
    Time {
      delta: Duration::ZERO,
      max_delta: Duration::from_hz(20),
    }
  }
}

#[derive(Debug, Default)]
pub struct UpdateTime {
  previous: Option<Instant>,
}

impl UpdateTime {
  pub fn new() -> Self {
    UpdateTime::default()
  }
}

impl<'a> ecs::System<'a> for UpdateTime {
  type SystemData = ecs::WriteResource<'a, Time>;

  fn setup(&mut self, res: &mut ecs::Resources) {
    res.entry().or_insert_with(Time::default);
  }

  fn run(&mut self, mut time: ecs::WriteResource<'a, Time>) {
    let now = Instant::now();

    time.delta = match self.previous {
      Some(previous) => now - previous,
      None => Duration::ZERO,
    };

    if time.delta > time.max_delta {
      time.delta = time.max_delta;
    }

    self.previous = Some(now);
  }
}
