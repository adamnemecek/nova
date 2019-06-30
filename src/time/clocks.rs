// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Tracks the time between calls to a `tick()` method.
///
/// This can be used to track the “delta time” or duration of time between two
/// iterations of a loop.
#[derive(Default)]
pub struct Clock {
  ticked_at: Option<Instant>,
  elapsed: Duration,
}

impl Clock {
  /// Creates a new clock with a fixed duration of zero.
  pub fn new() -> Self {
    Self::default()
  }

  /// Returns the amount of time elapsed between the most recent tick and the
  /// tick immediately before it.
  ///
  /// This is also commonly known as “delta time”.
  pub fn elapsed(&self) -> Duration {
    self.elapsed
  }

  /// Returns the instant in time of the most recent tick.
  ///
  /// # Panics
  ///
  /// This function panics if the clock has not yet been ticked.
  pub fn ticked_at(&self) -> Instant {
    self.ticked_at.expect("clock has not been ticked")
  }

  /// Updates the clock based on the time elapsed since the previous call to
  /// this method.
  pub fn tick(&mut self) {
    let now = Instant::now();

    if let Some(then) = self.ticked_at {
      self.elapsed = now - then;
    }

    self.ticked_at = Some(now);
  }
}

/// A [`Clock`] that tries to maintain a fixed duration fixed duration.
#[derive(Debug)]
pub struct FixedClock {
  fixed_duration: Duration,
  ticked_at: Option<Instant>,
  elapsed: Duration,
  elapsed_durations: usize,
  early: Duration,
  late: Duration,
}

impl FixedClock {
  /// Creates a new clock with the given fixed duration.
  pub fn new(fixed_duration: Duration) -> Self {
    debug_assert!(fixed_duration > Duration::ZERO, "fixed duration must be greater than zero");

    Self {
      fixed_duration,
      ticked_at: None,
      elapsed: Default::default(),
      elapsed_durations: 0,
      early: Default::default(),
      late: Default::default(),
    }
  }

  /// Returns the fixed duration of the clock.
  pub fn fixed_duration(&self) -> Duration {
    self.fixed_duration
  }

  /// Returns the amount of time elapsed between the most recent tick and the
  /// tick before it.
  ///
  /// This is also commonly known as “delta time”. It is always a multiple of
  /// the fixed duration as of the previous tick.
  pub fn elapsed(&self) -> Duration {
    self.elapsed
  }

  /// Returns the number of whole fixed durations of time that elapsed between
  /// the most recent tick and the tick before it.
  ///
  /// This is usually `1`, but may be larger if enough time elapsed between the
  /// last two ticks.
  pub fn elapsed_durations(&self) -> usize {
    self.elapsed_durations
  }

  /// Returns the instant in time of the most recent tick.
  ///
  /// # Panics
  ///
  /// This function panics if the clock has not yet been ticked.
  pub fn ticked_at(&self) -> Instant {
    self.ticked_at.expect("clock has not been ticked")
  }

  /// Updates the clock based on the time elapsed since the previous call to
  /// this method.
  ///
  /// This method will try to keep ticks spaced evenly according to the current
  /// fixed duration, and will sleep or spin the current thread to do so.
  pub fn tick(&mut self) {
    // Get the instant of the last tick or return immediately if this is the
    // first tick.
    let then = match self.ticked_at {
      Some(then) => then,

      None => {
        self.ticked_at = Some(Instant::now());
        return;
      }
    };

    // Compute the ideal duration of time since the previous tick, adjusting the
    // fixed duration according to how early or late the previous tick was.
    let ideal_duration = self.fixed_duration + self.early - self.late;

    // Compute the actual time elapsed since the previous tick.
    let mut now = Instant::now();
    let mut elapsed = now - then;

    // If less time has elapsed than the ideal duration, spin or sleep until it
    // has.
    if elapsed < ideal_duration {
      spin_sleep(ideal_duration - elapsed);

      now = Instant::now();
      elapsed = now - then;
    }

    self.ticked_at = Some(now);

    // Compute the number of whole fixed durations that have elapsed since the
    // previous tick.
    let elapsed_durations = (elapsed.as_secs() / self.fixed_duration.as_secs()).max(1.0);

    self.elapsed_durations = elapsed_durations as usize;

    // Compute the adjusted elapsed time which is always a multiple of the
    // fixed duration.
    self.elapsed = self.fixed_duration * elapsed_durations;

    // Store the difference between the real and adjusted elapsed time in the
    // early and late durations for next frame.
    self.early = self.elapsed - elapsed;
    self.late = elapsed - self.elapsed;
  }
}
