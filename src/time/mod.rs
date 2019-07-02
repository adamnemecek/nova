// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod clock;
mod duration;
mod instant;

pub use self::{clock::*, duration::*, instant::*};

use super::*;

pub fn sleep(duration: Duration) {
  std::thread::sleep(duration.try_into().expect("could not sleep for given duration"));
}

pub fn spin_sleep(duration: Duration) {
  spin_sleep::sleep(duration.try_into().expect("could not spin-sleep for given duration"));
}
