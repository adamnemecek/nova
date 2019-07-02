// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Tracks the “delta time” between calls to its `tick()` method.
#[derive(Default)]
pub struct Clock {
  is_synchronized: bool,
  ticked_at: Option<Instant>,
  delta_time: Duration,
  fixed_delta_time: Duration,
  fixed_intervals: usize,
  fixed_remainder: Duration,
}

impl Clock {
  /// Creates a new clock with a fixed duration of zero.
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets whether the clock will attempt to synchronize the delta time and
  /// fixed delta time durations by blocking the current thread if the clock
  /// is ticking too quickly.
  pub fn set_synchronized(&mut self, value: bool) {
    self.is_synchronized = value;
  }

  /// Returns the fixed delta time of the clock, or a zero-length duration if
  /// none is set.
  pub fn fixed_delta_time(&self) -> Duration {
    self.fixed_delta_time
  }

  /// Returns the duration of time between the previous two ticks.
  pub fn delta_time(&self) -> Duration {
    self.fixed_delta_time
  }

  /// Returns the number of whole fixed delta time intervals between the
  /// previous two ticks.
  ///
  /// If the fixed delta time is zero, this is always `0`.
  pub fn fixed_intervals(&self) -> usize {
    self.fixed_intervals
  }

  /// Sets the duration of a fixed delta time interval.
  ///
  /// Set to a zero-length duration to disable fixed interval tracking and
  /// synchronization.
  pub fn set_fixed_delta_time(&mut self, value: Duration) {
    self.fixed_delta_time = value;

    if value == Duration::ZERO {
      self.fixed_intervals = 0;
      self.fixed_remainder = Duration::ZERO;
    } else {
      self.update_fixed_intervals();
    }
  }

  /// Updates the clock with a new delta time.
  ///
  /// This function will block if the clock is synchronized and less than the
  /// fixed delta time has elapsed since the previous tick.
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

    // Compute the ideal delta time, which is the time remaining until another
    // fixed delta time interval should occur.
    let ideal_delta_time = self.fixed_delta_time - self.fixed_remainder;

    // Compute the actual delta time.
    let mut now = Instant::now();
    let mut delta_time = now - then;

    // If the clock is synchronized, try to wait for the ideal delta time to
    // elapse.
    if self.is_synchronized && delta_time < ideal_delta_time {
      spin_sleep(ideal_delta_time - delta_time);

      now = Instant::now();
      delta_time = now - then;
    }

    self.ticked_at = Some(now);
    self.delta_time = delta_time;

    if self.fixed_delta_time > Duration::ZERO {
      self.update_fixed_intervals();
    }
  }

  fn update_fixed_intervals(&mut self) {
    let delta_time = self.delta_time + self.fixed_remainder;
    let fixed_intervals = (delta_time.as_secs() / self.fixed_delta_time.as_secs()).floor();

    self.fixed_intervals = fixed_intervals as usize;

    // Calculated and store adjustments for the next frame.
    let adjusted_time = self.fixed_delta_time * fixed_intervals;

    self.fixed_remainder = delta_time - adjusted_time;
  }
}

/// A value indicating whether a loop should break and stop or continue and
/// run again.
pub enum LoopFlow {
  /// Break the loop.
  Break,
  /// Continue with the next iteration.
  Continue,
}

/// A join handle for a loop thread spawned with [`spawn_clock_loop`].
pub type LoopJoinHandle<'a> = thread::ScopedJoinHandle<'a, ()>;

/// Spawns a new thread and runs `tick_fn` in a loop with a [`Clock`].
///
/// The clock is synchronized and has its fixed delta time initalized to the
/// value of `interval`, meaning the loop will execute at that interval or as
/// near to it as possible.
pub fn spawn_clock_loop<'a>(
  scope: &'a thread::Scope,
  interval: Duration,
  mut tick_fn: impl FnMut(&mut Clock) -> LoopFlow + Send + 'static,
) -> thread::ScopedJoinHandle<'a, ()> {
  let mut clock = Clock::new();

  clock.set_fixed_delta_time(interval);
  clock.set_synchronized(true);

  scope.spawn(move |_| loop {
    clock.tick();

    match tick_fn(&mut clock) {
      LoopFlow::Break => break,
      LoopFlow::Continue => continue,
    }
  })
}
