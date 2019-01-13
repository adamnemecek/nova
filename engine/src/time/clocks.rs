// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::ecs::prelude::*;
use crate::time;
use crate::EngineHandle;

pub fn init(engine: &EngineHandle) {
  engine.execute_mut(|ctx| {
    ctx.register::<Clock>();
  });
}

pub fn tick(engine: &EngineHandle, delta_time: time::Duration) {
  engine.execute(|ctx| {
    ctx.run_system(&mut ClockTicker { delta_time });
  });
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Clock {
  pub time: time::Duration,
}

pub struct ClockTicker {
  pub delta_time: time::Duration,
}

impl<'a> System<'a> for ClockTicker {
  type Data = WriteComponents<'a, Clock>;

  fn run(&mut self, mut clocks: Self::Data) {
    for clock in (&mut clocks).join() {
      clock.time += self.delta_time;
    }
  }
}
