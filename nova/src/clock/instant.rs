// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Duration;
use derive_more::*;
use std::ops::Sub;
use std::time::Instant as StdInstant;

/// Represents an instant in time.
#[derive(Debug, Clone, Copy, From)]
pub struct Instant(StdInstant);

impl Instant {
  /// Gets a representation of the current instant in time.
  pub fn now() -> Self {
    StdInstant::now().into()
  }
}

// Implement `Sub` to yield a `Duration` when subtracting two instants.
impl Sub for Instant {
  type Output = Duration;

  fn sub(self, rhs: Self) -> Self::Output {
    (self.0 - rhs.0).into()
  }
}
