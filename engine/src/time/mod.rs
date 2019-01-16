// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod duration;
mod instant;
mod source;
mod ticker;

pub use self::duration::*;
pub use self::instant::*;
pub use self::source::*;
pub use self::ticker::*;

use crate::ecs;

pub fn tick(res: &ecs::Resources, delta_time: Duration) {
  ecs::run(&res, &mut Ticker::new(delta_time));
}

#[derive(Debug, Default)]
pub struct Time {
  pub ticks: u64,
  pub total: Duration,
  pub delta: Duration,
}

impl Time {
  pub fn tick(&mut self, delta: Duration) {
    self.delta = delta;
    self.total += delta;
    self.ticks += 1;
  }
}

pub fn delta(res: &ecs::Resources) -> Duration {
  res.fetch::<Time>().delta
}
