// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Duration, Instant};
use std::thread;

/// Remaining duration before the rate limiter should stop using
/// `thread::sleep()`.
///
/// This is set to 2.5ms because the rate limiter sleeps in 1ms increments,
/// but sleep timeouts aren't very accurate.
const SLEEP_UNTIL: Duration = Duration::from_micros(2500);

/// Remaining duration before the rate limiter should stop using
/// `thread::yield_now()`.
///
/// This is set to 25μs to get “close enough” without a high chance of going
/// over the target duration when the OS doesn't schedule the thread fast
/// enough.
const YIELD_UNTIL: Duration = Duration::from_micros(25);

/// Limits the rate at which an operation is completed.
///
/// This can be used to limit the rate of a fast loop, such as limiting the main
/// game loop to 60 Hz for a fixed time step, or polling for input at 144 Hz to
/// save CPU time.
#[derive(Debug)]
pub struct RateLimiter {
  timer_began: Instant,
}

impl RateLimiter {
  /// Creates a new frame limiter with the given target FPS.
  pub fn new() -> Self {
    RateLimiter {
      timer_began: Instant::now(),
    }
  }

  /// Begin the timer now. The next call to [`wait_for_min_duration()`] will be
  /// based on this start time.
  pub fn begin(&mut self) {
    self.timer_began = Instant::now();
  }

  /// Blocks the current thread until the given duration of time has elapsed
  /// since the previous call to [`begin()`].
  ///
  /// If [`begin()`] has not been called, the time since the rate limiter was
  /// created is used instead.
  pub fn wait_until(&mut self, duration: Duration) {
    // Repeatedly sleep until `SLEEP_UNTIL` time is left before the duration
    // elapses.
    let threshold = duration - SLEEP_UNTIL;

    while Instant::now() - self.timer_began < threshold {
      thread::sleep(std::time::Duration::from_millis(1));
    }

    // Repeatedly yield until `YIELD_UNTIL` time is left before the duration
    // elapses.
    let threshold = duration - YIELD_UNTIL;

    while Instant::now() - self.timer_began < threshold {
      thread::yield_now();
    }
  }
}

impl Default for RateLimiter {
  fn default() -> Self {
    Self::new()
  }
}
