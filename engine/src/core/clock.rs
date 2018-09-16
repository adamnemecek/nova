// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Tick;

/// Resource that keeps track of elapsed time.
#[derive(Default, Debug)]
pub struct Clock {
  /// Current tick.
  pub tick: Tick,
  /// Current time since launch in seconds.
  pub time: f64,
  /// Average FPS over the last 200 frames.
  pub fps: f64,
  /// Time elapsed in seconds between this tick and the last.
  pub delta_time: f64,
}
