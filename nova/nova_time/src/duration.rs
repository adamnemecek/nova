// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;
use std::ops::Sub;
use std::time::{Duration as StdDuration, Instant as StdInstant};
use std::u64;

/// Represents a span of time.
///
/// The standard library `Duration` stores time as a discrete, integer number of
/// seconds and nanoseconds. This structure instead stores time as a 64-bit
/// floating point number of seconds for easier use in time-dependent
/// calculations.
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Duration(f64);

impl Duration {
  /// Creates a `Duration` from a given number of seconds.
  ///
  /// This function will panic if the given value is not finite, is less than
  /// zero, or is too large to be represented by a standard libary `Duration`.
  pub fn from_secs(value: f64) -> Self {
    const MAX_SECS: f64 = u64::MAX as f64 + 1.0;

    if !value.is_finite() {
      panic!("got a non-finite number of seconds when creating a duration");
    }

    if value >= MAX_SECS {
      panic!("overflow converting number of seconds to duration");
    }

    if value < 0.0 {
      panic!("got a negative number of seconds when creating a duration");
    }

    Duration(value)
  }

  /// Converts the duration to a fractional number of seconds.
  ///
  /// Because this is how the duration is represented internally, this function
  /// does not actually need to perform any conversion.
  pub fn as_secs(self) -> f64 {
    self.0
  }
}

impl From<Duration> for StdDuration {
  fn from(value: Duration) -> Self {
    let nanos = (value.0 * 1_000_000_000.0) as u128;

    StdDuration::new(
      (nanos / 1_000_000_000) as u64,
      (nanos % 1_000_000_000) as u32,
    )
  }
}

impl From<StdDuration> for Duration {
  fn from(value: StdDuration) -> Self {
    Self(value.as_nanos() as f64 / 1_000_000_000.0)
  }
}

impl Sub<Duration> for Duration {
  type Output = Duration;

  fn sub(self, other: Self) -> Self::Output {
    Self((self.0 - other.0).max(0.0))
  }
}

impl fmt::Debug for Duration {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let secs = self.0;

    if secs < 0.000_000_000_001 {
      write!(f, "0")
    } else if secs < 0.000_001 {
      write!(f, "{:.3}ns", secs * 1_000_000_000.0)
    } else if secs < 0.001 {
      write!(f, "{:.3}μs", secs * 1_000_000.0)
    } else if secs < 1.0 {
      write!(f, "{:.3}ms", secs * 1_000.0)
    } else if secs < 60.0 {
      write!(f, "{:.3}s", secs * 1_000.0)
    } else if secs < 3_600.0 {
      write!(f, "{:.3}m", secs / 60.0)
    } else if secs < 86_400.0 {
      write!(f, "{:.3}h", secs / 3_600.0)
    } else {
      write!(f, "{:.3}d", secs / 86_400.0)
    }
  }
}
