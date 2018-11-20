// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;
use std::ops::Div;
use std::time::Duration as StdDuration;

use derive_more::*;

/// Represents a duration of time.
#[derive(Copy, Clone, PartialEq, PartialOrd, Add, AddAssign, Sub, SubAssign, Neg, Default)]
pub struct Duration(f64);

impl Duration {
  pub const ONE_SEC: Self = Duration(1.0);
  pub const ONE_60TH_SEC: Self = Duration(1.0 / 60.0);
  pub const ONE_120TH_SEC: Self = Duration(1.0 / 120.0);
  pub const ONE_144TH_SEC: Self = Duration(1.0 / 144.0);
  pub const ONE_MILLI: Self = Duration(0.001);

  /// Creates a new duration from the given number of seconds.
  ///
  /// Durations can only store finite, non-negative amounts of time.
  pub fn from_secs(secs: f64) -> Self {
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

// Implement `Div` so durations can be divided by a scalar dividend.
impl Div<f64> for Duration {
  type Output = Self;

  fn div(self, rhs: f64) -> Self {
    Duration::from_secs(self.0 / rhs)
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
