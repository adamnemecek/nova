// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Color;
use std::fmt;
use std::io::{self, Write};

/// Appends contextual information to a log message in the form of formatted
/// key/value pairs.
///
/// The value of `Default::default()` does no formatting and will output
/// nothing. This should be used with filtered log levels so no unneccessary
/// formatting is done on values that won't be displayed.
#[derive(Default)]
pub struct ContextBuilder<'a> {
  out: Option<io::StdoutLock<'a>>,
}

impl<'a> ContextBuilder<'a> {
  /// Creates a new builder writing to the given locked stdout.
  pub fn new(out: io::StdoutLock<'a>) -> Self {
    ContextBuilder { out: Some(out) }
  }

  /// Appends contextual information to the log message with a given key and
  /// value.
  pub fn with(&mut self, key: impl fmt::Display, value: impl fmt::Debug) -> &mut Self {
    if let Some(ref mut out) = self.out {
      write!(
        out,
        " {}{}:{} {:?}{}",
        Color::Cyan,
        key,
        Color::BrightCyan,
        value,
        Color::Reset
      )
      .ok();
    }

    self
  }
}

// Implement `Drop` to finish the log message by writing out a newline.
impl<'a> Drop for ContextBuilder<'a> {
  fn drop(&mut self) {
    if let Some(mut out) = self.out.take() {
      writeln!(out).unwrap();
    }
  }
}
