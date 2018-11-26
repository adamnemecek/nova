// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Duration, Instant};
use crate::utils::lazy_static;
use std::thread;

lazy_static! {
  static ref SLEEP_UNTIL: Duration = Duration::ONE_MILLI * 2.5;
  static ref YIELD_UNTIL: Duration = Duration::ONE_MICRO * 25.0;
}

/// Limits the rate at which an operation completes by ensuring that it takes a
/// certain minimum amount of time.
#[derive(Debug)]
pub struct RateLimiter {
  started: Instant,
}

impl RateLimiter {
  /// Creates a new frame limiter with the given target FPS.
  pub fn new() -> Self {
    RateLimiter {
      started: Instant::now(),
    }
  }

  pub fn begin(&mut self) {
    self.started = Instant::now();
  }

  pub fn wait_for_min_duration(&mut self, min: Duration) {
    let threshold = min - *SLEEP_UNTIL;

    while Instant::now() - self.started < threshold {
      thread::sleep(std::time::Duration::from_millis(1));
    }

    let threshold = min - *YIELD_UNTIL;

    while Instant::now() - self.started < threshold {
      thread::yield_now();
    }
  }
}
