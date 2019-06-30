// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod clock;
mod duration;
mod instant;

pub use self::{clock::*, duration::*, instant::*};

use super::*;

pub fn sleep(duration: Duration) {
  thread::sleep(duration.into());
}

pub fn spin_sleep(duration: Duration) {
  spin_sleep::sleep(duration.into());
}
