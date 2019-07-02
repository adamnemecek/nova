// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Represents a span of time.
///
/// The standard library `Duration` stores time as a discrete, integer number of
/// seconds and nanoseconds. This structure instead stores time as a 64-bit
/// floating point number of seconds for easier use in time-dependent
/// calculations.
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Duration {
  seconds: f64,
}

impl Duration {
  /// A zero-length duration.
  pub const ZERO: Duration = Duration { seconds: 0.0 };

  /// Converts the duration to a fractional number of seconds.
  ///
  /// Because this is how the duration is represented internally, this function
  /// does not actually need to perform any conversion.
  pub fn as_secs(self) -> f64 {
    self.seconds
  }
}

// Implement conversion to and from standard library durations.
impl From<std::time::Duration> for Duration {
  fn from(value: std::time::Duration) -> Self {
    Self { seconds: value.as_nanos() as f64 / 1_000_000_000.0 }
  }
}

impl TryFrom<Duration> for std::time::Duration {
  type Error = &'static str;

  fn try_from(value: Duration) -> Result<Self, Self::Error> {
    const MAX_SECONDS: f64 = u64::MAX as f64 + 1.0;

    if value.seconds > MAX_SECONDS {
      return Err("too long to store in a std::time::Duration");
    }

    let nanos = (value.seconds * 1_000_000_000.0) as u128;

    Ok(std::time::Duration::new((nanos / 1_000_000_000) as u64, (nanos % 1_000_000_000) as u32))
  }
}

// Implement operators for working with durations.
impl ops::Add<Duration> for Duration {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self { seconds: self.seconds + other.seconds }
  }
}

impl ops::AddAssign<Duration> for Duration {
  fn add_assign(&mut self, other: Self) {
    self.seconds += other.seconds
  }
}

impl ops::Sub<Duration> for Duration {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self { seconds: (self.seconds - other.seconds).max(0.0) }
  }
}

impl ops::Rem<Duration> for Duration {
  type Output = Self;

  fn rem(self, other: Self) -> Self::Output {
    seconds(self.seconds % other.seconds)
  }
}

impl ops::Mul<f64> for Duration {
  type Output = Self;

  fn mul(self, value: f64) -> Self::Output {
    seconds(self.seconds * value)
  }
}

// Implement formatting and logging traits to display human-readable durations.
impl fmt::Debug for Duration {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let secs = self.seconds;

    if secs < 0.000_000_000_001 {
      write!(f, "0")
    } else if secs < 0.000_001 {
      write!(f, "{:.3}ns", secs * 1_000_000_000.0)
    } else if secs < 0.001 {
      write!(f, "{:.3}μs", secs * 1_000_000.0)
    } else if secs < 1.0 {
      write!(f, "{:.3}ms", secs * 1_000.0)
    } else if secs < 60.0 {
      write!(f, "{:.3}s", secs)
    } else if secs < 3_600.0 {
      write!(f, "{:.3}m", secs / 60.0)
    } else if secs < 86_400.0 {
      write!(f, "{:.3}h", secs / 3_600.0)
    } else {
      write!(f, "{:.3}d", secs / 86_400.0)
    }
  }
}

impl fmt::Display for Duration {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fmt::Debug::fmt(self, f)
  }
}

impl log::Value for Duration {
  fn serialize(
    &self,
    _: &log::Record,
    key: log::Key,
    serializer: &mut dyn log::Serializer,
  ) -> log::SerializationResult {
    serializer.emit_arguments(key, &format_args!("{}", self))
  }
}

/// Returns a value representing a duration of `1.0 / hz` seconds.
///
/// # Panics
///
/// This function will panic if `hz` is a negative number.
pub fn hz(hz: f64) -> Duration {
  seconds(1.0 / hz)
}

/// Returns a value representing a duration of the given number of seconds.
///
/// # Panics
///
/// This function will panic if `seconds` is a negative number.
pub fn seconds(seconds: f64) -> Duration {
  if seconds < 0.0 {
    panic!("durations cannot be negative");
  }

  Duration { seconds }
}
