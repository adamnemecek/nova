// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;

/// One of the possible terminal colors.
#[allow(dead_code)]
pub enum Color {
  Black,
  Red,
  Green,
  Yellow,
  Blue,
  Magenta,
  Cyan,
  White,
  BrightBlack,
  BrightRed,
  BrightGreen,
  BrightYellow,
  BrightBlue,
  BrightMagenta,
  BrightCyan,
  BrightWhite,
  Reset,
}

// Display the color by writing an escape sequence.
impl fmt::Display for Color {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    formatter.write_str(match self {
      Color::Black => "\x1b[0m\x1b[30m",
      Color::Red => "\x1b[0m\x1b[31m",
      Color::Green => "\x1b[0m\x1b[32m",
      Color::Yellow => "\x1b[0m\x1b[33m",
      Color::Blue => "\x1b[0m\x1b[34m",
      Color::Magenta => "\x1b[0m\x1b[35m",
      Color::Cyan => "\x1b[0m\x1b[36m",
      Color::White => "\x1b[0m\x1b[37m",
      Color::BrightBlack => "\x1b[30;1m",
      Color::BrightRed => "\x1b[31;1m",
      Color::BrightGreen => "\x1b[32;1m",
      Color::BrightYellow => "\x1b[33;1m",
      Color::BrightBlue => "\x1b[34;1m",
      Color::BrightMagenta => "\x1b[35;1m",
      Color::BrightCyan => "\x1b[36;1m",
      Color::BrightWhite => "\x1b[37;1m",
      Color::Reset => "\x1b[0m",
    })
  }
}
