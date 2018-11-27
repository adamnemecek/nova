// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Duration, Instant, Settings};
use crate::Engine;

/// A resource that stores time information for an engine loop.
pub struct Clock {
  /// The instant the clock was last updated.
  pub updated_at: Instant,
  /// The total duration of time that has elapsed since the clock was
  /// initialized.
  pub time: Duration,
  /// The duration of time that elapsed between the most recent two clock
  /// updates.
  pub delta_time: Duration,
}

impl Clock {
  /// Creates a new clock with no elapsed time.
  pub fn new() -> Self {
    Clock {
      updated_at: Instant::now(),
      time: Duration::ZERO,
      delta_time: Duration::ZERO,
    }
  }

  /// Updates the clock using the given settings.
  ///
  /// Note that if the duration of time since the last clock update is shorter
  /// than the set `min_delta_time`, the clock will update faster than real time.
  /// Use a [`RateLimiter`] or other external method to prevent this.
  pub fn update(&mut self, settings: &Settings) {
    let now = Instant::now();

    let delta_time = match now - self.updated_at {
      x if x > settings.max_delta_time => settings.max_delta_time,
      x if x < settings.min_delta_time => settings.min_delta_time,
      x => x,
    };

    self.updated_at = now;
    self.delta_time = delta_time;
    self.time += delta_time;
  }
}

impl Default for Clock {
  fn default() -> Self {
    Clock::new()
  }
}

/// Updates the [`Clock`] resource of the given engine instance with the current
/// time.
///
/// The clock is updated using the settings in the [`Settings`] resource.
///
/// Note that if the duration of time since the last clock update is shorter
/// than the set `min_delta_time`, the clock will update faster than real time.
/// Use a [`RateLimiter`] or other external method to prevent this.
pub fn update_clock(engine: &mut Engine) {
  let settings = engine.fetch_resource::<Settings>();
  let mut clock = engine.fetch_resource_mut::<Clock>();

  clock.update(&settings);
}
