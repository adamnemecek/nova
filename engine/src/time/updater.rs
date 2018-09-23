// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Clock;
use crate::prelude::*;
use std::time::Instant;

/// System that updates the time stored in `Clock`.
#[derive(Default)]
pub struct Updater {
  /// Instant the clock was last updated or `None` if it never was.
  last_update: Option<Instant>,
}

impl<'a> System<'a> for Updater {
  type SystemData = WriteResource<'a, Clock>;

  fn run(&mut self, mut clock: Self::SystemData) {
    let now = Instant::now();

    clock.delta_time = match self.last_update {
      Some(time) => ggez::timer::duration_to_f64(now - time),
      None => 0.0,
    };

    clock.time += clock.delta_time;

    self.last_update = Some(now);
  }
}
