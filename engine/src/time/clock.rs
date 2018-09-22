// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Resource that stores engine time info.
#[derive(Debug)]
pub struct Clock {
  /// Total time elapsed on the clock.
  pub time: f64,
  /// Time elapsed between the latest tick and the previous tick.
  pub delta_time: f64,
}

// Sets up the default clock up for the first tick.
impl Default for Clock {
  fn default() -> Self {
    Clock {
      time: 0.0,
      delta_time: 0.0,
    }
  }
}
