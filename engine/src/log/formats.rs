// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;

/// Formats a log message context value using its [`fmt::Display`]
/// implementation.
pub struct Display<T: fmt::Display>(pub T);

impl<T: fmt::Display> fmt::Debug for Display<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "{}", self.0)
  }
}
