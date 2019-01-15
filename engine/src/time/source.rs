// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Duration, Instant};

pub trait Source: std::fmt::Debug {
  fn delta_time(&mut self) -> Duration;
}

#[derive(Default, Debug)]
pub struct RealTime {
  previous: Option<Instant>,
}

impl RealTime {
  pub const fn new() -> Self {
    RealTime { previous: None }
  }
}

impl Source for RealTime {
  fn delta_time(&mut self) -> Duration {
    let now = Instant::now();

    let delta = match self.previous {
      Some(previous) => now - previous,
      None => Duration::ZERO,
    };

    self.previous = Some(now);

    delta
  }
}

impl Source for Duration {
  fn delta_time(&mut self) -> Duration {
    *self
  }
}
