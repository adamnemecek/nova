// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Limits the frequency of a loop and tracks the time between each iteration.
#[derive(Default)]
pub struct Clock {
  ticked_at: Option<Instant>,
  elapsed: Duration,
}

impl Clock {
  /// Creates a new clock with a tick length of zero.
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

  /// Updates the clock.
  ///
  /// The first time this function is called, it sets the initial tick time of
  /// the clock and returns immediately. The `elapsed()` method will return a
  /// zero duration.
  ///
  /// On subsequent calls, this function will check if the time elapsed since
  /// the previous call is shorter than the tick length. If so, it will block
  /// for the remaining time. The `elapsed()` method will then return the actual
  /// duration of time elapsed since the previous call.
  pub fn tick(&mut self) {
    let mut now = Instant::now();

    match self.ticked_at {
      Some(then) => {
        self.elapsed = now - then;
      }

      None => {}
    };

    self.ticked_at = Some(now);
  }
}

#[derive(Debug)]
pub struct FixedClock {
  tick_length: Duration,
  ticked_at: Option<Instant>,
  elapsed: Duration,
  elapsed_ticks: usize,
  early: Duration,
  late: Duration,
}

impl FixedClock {
  pub fn new(tick_length: Duration) -> Self {
    debug_assert!(tick_length > Duration::ZERO, "tick length must be greater than zero");

    Self {
      tick_length,
      ticked_at: None,
      elapsed: Default::default(),
      elapsed_ticks: 0,
      early: Default::default(),
      late: Default::default(),
    }
  }

  /// Returns the target tick length of the clock.
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

  /// Returns the amount of time elapsed between the most recent tick and the
  /// tick immediately before it, as a who
  ///
  /// This is also commonly known as “delta time”.
  pub fn elapsed_ticks(&self) -> Duration {
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

  pub fn tick(&mut self) {
    let then = match self.ticked_at {
      Some(then) => then,

      None => {
        self.ticked_at = Some(Instant::now());
        return;
      }
    };

    let threshold = self.tick_length + self.early - self.late;

    let mut now = Instant::now();
    let mut elapsed = now - then;

    if elapsed < threshold {
      spin_sleep(threshold - elapsed);

      now = Instant::now();
      elapsed = now - then;
    }

    let elapsed_ticks = (elapsed.as_secs() / self.tick_length.as_secs()).max(1.0).round();

    self.elapsed = self.tick_length * elapsed_ticks;
    self.elapsed_ticks = elapsed_ticks as usize;
    self.early = self.elapsed - elapsed;
    self.late = elapsed - self.elapsed;
    self.ticked_at = Some(now);
  }
}
