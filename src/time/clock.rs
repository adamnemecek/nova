// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Limits the frequency of a loop and tracks the time between each iteration.
pub struct Clock {
  ticked_at: Instant,
  tick_length: Duration,
  elapsed: Duration,
}

impl Default for Clock {
  fn default() -> Self {
    Self::new()
  }
}

impl Clock {
  /// Creates a new clock with a tick length of zero.
  pub fn new() -> Self {
    Self { ticked_at: Instant::now(), tick_length: Default::default(), elapsed: Default::default() }
  }

  /// Returns the tick length of the clock.
  ///
  /// This is the minimum amount of time that must elapse between ticks.
  pub fn tick_length(&self) -> Duration {
    self.tick_length
  }

  /// Returns the amount of time elapsed between the most recent tick and the
  /// tick immediately before it.
  ///
  /// This is also commonly known as “delta time”.
  pub fn elapsed(&self) -> Duration {
    self.elapsed
  }

  /// Sets the tick length of the clock.
  ///
  /// This is the minimum amount of time that must elapse between ticks.
  pub fn set_tick_length(&mut self, length: Duration) {
    self.tick_length = length;
  }

  /// Sets the tick length of the clock.
  ///
  /// This is the minimum amount of time that must elapse between ticks.
  pub fn with_tick_length(mut self, length: Duration) -> Self {
    self.set_tick_length(length);
    self
  }

  /// Sets the frequency of clock ticks.
  ///
  /// This is equivalent to setting the tick length to `1 / value` seconds.
  pub fn set_frequency(&mut self, value: f64) {
    self.tick_length = if value.is_infinite() && value > 0.0 {
      Duration::default()
    } else {
      Duration::from_secs(1.0 / value)
    };
  }

  /// Sets the frequency of clock ticks.
  ///
  /// This is equivalent to setting the tick length to `1 / value` seconds.
  pub fn with_frequency(mut self, value: f64) -> Self {
    self.set_frequency(value);
    self
  }

  /// Updates the clock.
  ///
  /// If less time has elapsed than the tick length of the clock, this function
  /// will block until the remaining time has elapsed.
  pub fn tick(&mut self) {
    let mut now = Instant::now();
    let mut elapsed = now - self.ticked_at;

    if elapsed < self.tick_length {
      spin_sleep::sleep((self.tick_length - elapsed).into());

      now = Instant::now();
      elapsed = now - self.ticked_at;
    }

    self.ticked_at = now;
    self.elapsed = elapsed;
  }
}
