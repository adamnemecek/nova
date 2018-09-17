// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Number that is incremented once per game loop, uniquely identifying one
/// iteration of the loop.
pub type Tick = u64;

/// Resource that stores engine time info.
#[derive(Default, Debug)]
pub struct Clock {
  /// Current tick, a number that is incremented each game loop.
  pub tick: Tick,
  /// Current time since the engine started in seconds.
  pub time: f64,
  /// Average FPS over the last 200 ticks.
  pub fps: f64,
  /// Time in seconds between the current tick and the last tick.
  pub delta_time: f64,
}
