// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod update;

mod duration;
mod instant;

pub use self::duration::Duration;
pub use self::instant::Instant;
pub use self::update::UpdateClock;

use crate::resources::{self, ReadResource, Resources, WriteResource};

pub type ReadClock<'a> = ReadResource<'a, Clock>;
pub type WriteClock<'a> = WriteResource<'a, Clock>;

#[derive(Debug, Default)]
pub struct Clock {
  pub delta_time: Duration,
}

pub fn borrow(res: &Resources) -> ReadClock {
  resources::borrow(res)
}

pub fn borrow_mut(res: &Resources) -> WriteClock {
  resources::borrow_mut(res)
}
