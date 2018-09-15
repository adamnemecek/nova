// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{buttons, Button};

#[derive(Default, Debug)]
pub struct State {
  pub buttons: [ButtonState; buttons::COUNT],
}

#[derive(Default, Debug)]
pub struct ButtonState {
  pub pressed_time: Option<f64>,
  pub repeated: bool,
}

impl State {
  /// Returns `true` while the given `button` is held down.
  pub fn is_pressed(&self, button: Button) -> bool {
    self.buttons[button as usize].pressed_time.is_some()
  }

  /// Returns `true` on the first frame the given `button` is pressed and
  /// periodically while it is held down.
  ///
  /// This is useful for discrete actions that should repeatedly happen while a
  /// button is held, such as selecting menu items with arrow keys.
  pub fn is_pressed_repeated(&self, button: Button) -> bool {
    self.buttons[button as usize].repeated
  }
}
