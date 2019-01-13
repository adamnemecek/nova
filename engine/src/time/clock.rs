// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Duration, Instant, Settings};

/// A resource that stores time information for an engine loop.
pub struct Clock {
  /// The instant the clock was last ticked.
  pub ticked_at: Instant,
  /// The total duration of time that has elapsed since the clock was
  /// initialized.
  pub total_time: Duration,
  /// The duration of time that elapsed between the most recent two clock
  /// updates.
  pub delta_time: Duration,
}

impl Clock {
  /// Creates a new clock with no elapsed time.
  pub fn new() -> Self {
    Clock {
      ticked_at: Instant::now(),
      total_time: Duration::ZERO,
      delta_time: Duration::ZERO,
    }
  }

  /// Updates the clock using the given settings.
  pub fn tick(&mut self, settings: &Settings) {
    let now = Instant::now();

    let delta_time = match now - self.ticked_at {
      x if x > settings.max_delta_time => settings.max_delta_time,
      x if x < settings.min_delta_time => settings.min_delta_time,
      x => x,
    };

    self.ticked_at = now;
    self.delta_time = delta_time;
    self.total_time += delta_time;
  }
}

impl Default for Clock {
  fn default() -> Self {
    Clock::new()
  }
}
