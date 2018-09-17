// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use ggez::event::KeyCode;

/// Resource that stores the keyboard events from the current tick.
#[derive(Default)]
pub struct KeyEvents {
  /// List of all keyboard events from the current tick.
  pub list: Vec<KeyEvent>,
}

/// A single keyboard event.
#[derive(Debug)]
pub enum KeyEvent {
  /// Indicates that a key was pressed, or that it was held long enough for the
  /// press to repeat.
  Pressed(KeyCode),
  /// Indicates that a key was released.
  Released(KeyCode),
}
