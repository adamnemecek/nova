// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Duration;

/// Stores an elapsed duration of time.
#[derive(Default, Debug)]
pub struct Clock {
  /// The last duration elapsed on the clock with [`elapse()`].
  pub elapsed: Duration,
  /// The total duration elapsed on the clock.
  pub elapsed_total: Duration,
}

impl Clock {
  /// Creates a new clock with no elapsed time.
  pub fn new() -> Clock {
    Clock::default()
  }

  /// Adds a given duration to the total elapsed time on the clock.
  ///
  /// This will set [`elapsed`] equal to the given `delta` time.
  pub fn elapse(&mut self, delta: Duration) {
    self.elapsed = delta;
    self.elapsed_total += delta;
  }
}
