// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod update;

mod duration;
mod instant;

pub use self::duration::Duration;
pub use self::instant::Instant;
pub use self::update::UpdateClock;

use crate::ecs;

pub type ReadClock<'a> = ecs::ReadResource<'a, Clock>;
pub type WriteClock<'a> = ecs::WriteResource<'a, Clock>;

#[derive(Debug, Default)]
pub struct Clock {
  pub delta_time: Duration,
}

impl Clock {
  pub fn read(res: &ecs::Resources) -> ReadClock {
    ecs::SystemData::fetch(res)
  }

  pub fn write(res: &ecs::Resources) -> WriteClock {
    ecs::SystemData::fetch(res)
  }
}
