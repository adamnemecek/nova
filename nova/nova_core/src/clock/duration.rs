// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;
use std::ops::{Div, Mul, Sub, SubAssign};
use std::time::Duration as StdDuration;

use derive_more::*;

/// Represents a duration of time.
#[derive(Copy, Clone, PartialEq, PartialOrd, Add, AddAssign, Neg, Default)]
pub struct Duration(f64);

impl Duration {
  pub const ZERO: Self = Duration(0.0);

  pub fn from_secs(secs: u64) -> Self {
    Duration(secs as f64)
  }

  pub fn from_millis(millis: u64) -> Self {
    Duration(millis as f64 * 0.001)
  }

  pub fn from_micros(micros: u64) -> Self {
    Duration(micros as f64 * 0.000_001)
  }

  pub fn from_hz(hz: u64) -> Self {
    Duration(1.0 / hz as f64)
  }

  /// Creates a new duration from the given number of seconds.
  pub fn from_float_secs(secs: f64) -> Self {
    debug_assert!(secs.is_finite(), "Durations be finite.");

    debug_assert!(secs >= 0.0, "Durations must be non-negative.");

    debug_assert!(
      secs <= u64::max_value() as f64,
      "Durations must not exceed {} seconds.",
      u64::max_value()
    );

    Duration(secs)
  }

  /// Converts the duration to a number of seconds.
  pub fn as_secs(self) -> f64 {
    self.0
  }
}

// Implement `From` to convert to and from standard `Duration` structs.
impl From<Duration> for StdDuration {
  fn from(secs: Duration) -> Self {
    let nanos = secs.0 * 1e9;
    let nanos = nanos as u128;

    StdDuration::new(
      (nanos / 1_000_000_000) as u64,
      (nanos % 1_000_000_000) as u32,
    )
  }
}

impl From<StdDuration> for Duration {
  fn from(duration: StdDuration) -> Self {
    Duration((duration.as_secs() as f64) + f64::from(duration.subsec_nanos()) / 1e9)
  }
}

// Implement subtraction of durations to never go below 0.
impl Sub<Self> for Duration {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Duration((self.0 - rhs.0).max(0.0))
  }
}

impl SubAssign<Self> for Duration {
  fn sub_assign(&mut self, rhs: Self) {
    self.0 = (self.0 - rhs.0).max(0.0);
  }
}

// Implement multiplication of durations with scalar values.
impl Mul<f64> for Duration {
  type Output = Duration;

  fn mul(self, rhs: f64) -> Self::Output {
    Duration::from_float_secs(self.0 * rhs)
  }
}

impl Div<f64> for Duration {
  type Output = Self;

  fn div(self, rhs: f64) -> Self {
    Duration::from_float_secs(self.0 / rhs)
  }
}

// Implement `fmt::Debug` to display the value using standard duration
// formatting, which shows nanoseconds, microseconds, milliseconds, or seconds
// depending on magnitude.
impl fmt::Debug for Duration {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let duration: StdDuration = self.clone().into();

    write!(f, "{:?}", duration)
  }
}

// Implement `fmt::Display` to delegate to `fmt::Debug`.
impl fmt::Display for Duration {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fmt::Debug::fmt(self, f)
  }
}
