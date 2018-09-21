// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `time` module provides shared time state.
//!
//! The `Clock` resource stores time information and must be updated once per
//! frame with the `tick` function.

use super::*;

pub use std::time::{Duration, Instant};

/// Resource that stores engine time info.
#[derive(Debug)]
pub struct Clock {
  /// Total time elapsed on the clock.
  pub time: f64,
  /// Time elapsed between the latest tick and the previous tick.
  pub delta_time: f64,
  /// Instant the clock was last ticked.
  ticked_at: Instant,
}

// Sets up the default clock up for the first tick.
impl Default for Clock {
  fn default() -> Self {
    Clock {
      time: 0.0,
      delta_time: 0.0,
      ticked_at: Instant::now(),
    }
  }
}

/// Sets up time for the given world.
pub fn setup(world: &mut World) {
  world.add_resource(Clock::default());
}

/// Updates time for the given world.
pub fn tick(world: &mut World) {
  let now = Instant::now();
  let mut clock = world.write_resource::<Clock>();

  clock.delta_time = ggez::timer::duration_to_f64(now - clock.ticked_at);
  clock.time += clock.delta_time;
  clock.ticked_at = now;
}
