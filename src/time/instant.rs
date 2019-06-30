// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use std::time::Instant as StdInstant;

/// Opaquely represents a specific instant in time.
///
/// Like the standard library `Instant`, this structure can be used to calculate
/// `Duration` values using the `elapsed()` method for example.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Instant(StdInstant);

impl Instant {
  /// Returns a value representing the current instant in time.
  pub fn now() -> Self {
    StdInstant::now().into()
  }

  /// Returns a value representing the duration of time elapsed since this
  /// instant in time.
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

impl ops::Sub<Instant> for Instant {
  type Output = Duration;

  fn sub(self, other: Self) -> Self::Output {
    (self.0 - other.0).into()
  }
}
