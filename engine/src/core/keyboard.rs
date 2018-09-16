// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use ggez::event::KeyCode;

/// Resource that contains all keyboard events that occurred this tick.
#[derive(Default)]
pub struct Events {
  /// List of all keyboard events that occured this tick.
  pub list: Vec<Event>,
}

/// Represents a single keyboard event.
#[derive(Debug)]
pub enum Event {
  /// Indicates that a key was pressed, or that it was held long enough for the
  /// press to repeat.
  Pressed(KeyCode),
  /// Indicates that a key was released.
  Released(KeyCode),
}
