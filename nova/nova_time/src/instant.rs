// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;
use std::ops::Sub;
use std::time::{Duration as StdDuration, Instant as StdInstant};
use std::u64;
use crate::Duration;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant(StdInstant);

impl Instant {
  pub fn now() -> Self {
    StdInstant::now().into()
  }

  pub fn elapsed(&self) -> Duration {
    self.0.elapsed().into()
  }
}

impl From<StdInstant> for Instant {
  fn from(value: StdInstant) -> Self {
    Self(value)
  }
}

impl From<Instant> for StdInstant {
  fn from(value: Instant) -> Self {
    value.0
  }
}

impl Sub<Instant> for Instant {
  type Output = Duration;

  fn sub(self, other: Self) -> Self::Output {
    (self.0 - other.0).into()
  }
}
