// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

pub use std::time::{Duration, Instant};

/// Resource that stores engine time info.
#[derive(Debug)]
pub struct Clock {
  /// Number of times the clock has been ticked.
  pub ticks: u64,
  /// Instant of time the clock was last ticked.
  pub ticked_at: Instant,
  /// Time between the latest tick and the previous tick.
  pub delta_time: Duration,
}

impl Default for Clock {
  fn default() -> Self {
    Clock {
      ticks: 0,
      ticked_at: Instant::now(),
      delta_time: time::Duration::default(),
    }
  }
}

pub fn setup(core: &mut Core) {
  core.world.add_resource(Clock::default());
}

pub fn tick(core: &mut Core) {
  let now = Instant::now();
  let mut clock = core.world.write_resource::<Clock>();

  clock.ticks += 1;
  clock.delta_time = now - clock.ticked_at;
  clock.ticked_at = now;
}

pub fn seconds(duration: Duration) -> f64 {
  let secs = duration.as_secs() as f64;

  secs + duration.subsec_nanos() as f64 / 1_000_000_000.0
}

pub fn seconds_f32(duration: Duration) -> f32 {
  let secs = duration.as_secs() as f32;

  secs + duration.subsec_nanos() as f32 / 1_000_000_000.0
}
