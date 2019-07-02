// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Tracks time across individual calls to a `tick()` function.
#[derive(Default, Debug)]
pub struct Clock {
  ticked_at: Option<Instant>,
  interval: Duration,
  intervals_synchronized: bool,
  elapsed: Duration,
  elapsed_delta: Duration,
  elapsed_intervals: u64,
  elapsed_intervals_delta: u64,
  elapsed_interval_remainder: Duration,
}

impl Clock {
  /// Creates a new clock.
  pub fn new() -> Self {
    Self::default()
  }

  /// Returns the instant and time when the clock was last ticked.
  ///
  /// # Panics
  ///
  /// This function panics if the clock has never been ticked.
  pub fn ticked_at(&self) -> Instant {
    self.ticked_at.expect("clock has never been ticked")
  }

  /// Returns the duration of an interval of the clock, or zero if no interval
  /// is set.
  pub fn interval(&self) -> Duration {
    self.interval
  }

  /// Sets the duration of an interval.
  ///
  /// If the interval is set to zero, interval tracking is disabled and the
  /// `tick()` method never blocks even if synchronizing intervals.
  pub fn set_interval(&mut self, value: Duration) {
    self.interval = value;
  }

  /// Sets whether or not to synchronize intervals.
  ///
  /// When set to `true`, the `tick()` will block the current thread if needed
  /// to attempt to align calls to the set interval. If no interval is set, this
  /// setting has no effect.
  pub fn set_intervals_synchronized(&mut self, value: bool) {
    self.intervals_synchronized = value;
  }

  /// Returns the total elapsed duration as of the most recent tick, or zero
  /// if the clock has not been ticked.
  pub fn elapsed(&self) -> Duration {
    self.elapsed
  }

  /// Returns the duration of time elapsed between the last two ticks, or zero
  /// if the clock has not been ticked twice.
  pub fn elapsed_delta(&self) -> Duration {
    self.elapsed
  }

  /// Returns the total number of intervals elapsed as of the most recent tick.
  ///
  /// This is zero if no intervals have elapsed or if the clock has never had
  /// a set interval when ticked.
  pub fn elapsed_intervals(&self) -> u64 {
    self.elapsed_intervals
  }

  /// Returns the number of intervals elapsed between tho two most recent ticks.
  ///
  /// This is zero if no intervals elapsed or if the clock did not have a set
  /// interval on the most recent tick.
  pub fn elapsed_intervals_delta(&self) -> u64 {
    self.elapsed_intervals_delta
  }

  /// Updates the values tracked by the clock based on the current time.
  ///
  /// If the clock is set to synchronize intervals, this method will block until
  /// roughly one interval has elapsed if one has not already. If the interval
  /// is zero, this method will never block and will not track intervals.
  pub fn tick(&mut self) {
    // Get the instant of the last tick or return immediately if this is the
    // first tick.
    let prev_ticked_at = match self.ticked_at {
      Some(then) => then,

      None => {
        self.ticked_at = Some(now());
        return;
      }
    };

    // Compute the ideal elapsed delta, which is the time remaining until
    // another interval elapses.
    let ideal_elapsed_delta = self.interval - self.elapsed_interval_remainder;

    // Compute the actual elapsed delta.
    let mut ticked_at = now();
    let mut elapsed_delta = ticked_at - prev_ticked_at;

    // If the clock is synchronized, try to wait for the ideal delta time to
    // elapse.
    if self.intervals_synchronized && elapsed_delta < ideal_elapsed_delta {
      spin_sleep(ideal_elapsed_delta - elapsed_delta);

      ticked_at = now();
      elapsed_delta = ticked_at - prev_ticked_at;
    }

    self.ticked_at = Some(ticked_at);

    self.elapsed_delta = elapsed_delta;
    self.elapsed += elapsed_delta;

    // If an interval is set, track the number of elapsed intervals.
    if self.interval > Duration::ZERO {
      elapsed_delta += self.elapsed_interval_remainder;

      let elapsed_intervals_delta = (elapsed_delta.as_secs() / self.interval.as_secs()).floor();

      self.elapsed_intervals_delta = elapsed_intervals_delta as u64;
      self.elapsed_interval_remainder = elapsed_delta - self.interval * elapsed_intervals_delta;
      self.elapsed_intervals += self.elapsed_intervals_delta;
    } else {
      self.elapsed_intervals = 0;
    }
  }
}
