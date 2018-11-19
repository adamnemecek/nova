// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use log::{Level, LevelFilter};

use super::Color;
use std::fmt;

/// Implements [`fmt::Display`] to show a colorized level for pretty logging.
pub struct PrettyLevel(pub Level);

impl fmt::Display for PrettyLevel {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match self.0 {
      Level::Trace => write!(f, "{}… TRCE{}", Color::White, Color::Reset),
      Level::Debug => write!(f, "{} DEBG{}", Color::BrightMagenta, Color::Reset),
      Level::Info => write!(f, "{} INFO{}", Color::BrightBlue, Color::Reset),
      Level::Warn => write!(f, "{} WARN{}", Color::BrightYellow, Color::Reset),
      Level::Error => write!(f, "{} ERRO{}", Color::BrightRed, Color::Reset),
    }
  }
}
