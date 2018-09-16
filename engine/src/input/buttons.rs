// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::keyboard::KeyCode;

/// One of the available input buttons.
#[derive(Debug)]
pub enum Button {
  Up,
  Left,
  Down,
  Right,
}

/// Total number of available input buttons.
pub const COUNT: usize = Button::Right as usize + 1;

impl Button {
  /// Converts a given `KeyCode` to a `Button`.
  ///
  /// Currently hard-coded for Neo2 keyboard layout. Sorry!
  pub fn from_keycode(key: &KeyCode) -> Option<Button> {
    match key {
      KeyCode::V => Some(Button::Up),
      KeyCode::U => Some(Button::Left),
      KeyCode::I => Some(Button::Down),
      KeyCode::A => Some(Button::Right),
      _ => None,
    }
  }
}
