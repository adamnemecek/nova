// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Duration, Instant};
use crate::ecs::Resources;

#[derive(Debug)]
pub struct Time {
  pub delta: Duration,
  pub max_delta: Duration,
}

impl Time {
  pub fn setup(res: &mut Resources) {
    res.entry().or_insert_with(|| Time {
      delta: Duration::ZERO,
      max_delta: Duration::from_hz(20),
    });
  }

  pub fn update(&mut self, delta_time: DeltaTime) {
    let now = Instant::now();

    self.delta = match delta_time {
      DeltaTime::Fixed(duration) => duration,
      DeltaTime::SincePrevious(Some(previous)) => now - *previous,
      DeltaTime::SincePrevious(None) => Duration::ZERO,
    };

    if self.delta > self.max_delta {
      self.delta = self.max_delta;
    }

    if let DeltaTime::SincePrevious(previous) = delta_time {
      *previous = Some(now);
    }
  }
}

#[derive(Debug)]
pub enum DeltaTime<'a> {
  Fixed(Duration),
  SincePrevious(&'a mut Option<Instant>),
}
