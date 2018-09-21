// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `input` module provides simple input functionality.
//!
//! Keyboard keys are mapped to virtual buttons in the `Button` enum
//! representing the available basic actions in the game. This mapping is
//! controlled by the `Mapping` resource which can be loaded from an asset file.
//!
//! The state of input buttons is stored in the `Input` resource, which must be
//! updated with the `update` function once per frame.

use super::*;
pub use ggez::event::KeyCode;

mod button;
mod mapping;
mod update;

pub use self::button::*;
pub use self::mapping::*;
pub use self::update::*;

/// Resource that stores the current input state.
#[derive(Default, Debug)]
pub struct Input {
  /// State of all available input buttons.
  pub buttons: [ButtonState; BUTTON_COUNT],
}

impl Input {
  /// Returns `true` while the given `button` is held down.
  pub fn is_pressed(&self, button: Button) -> bool {
    self.buttons[button as usize].pressed_at.is_some()
  }

  /// Returns `true` on the first frame the given `button` is pressed and
  /// periodically while it is held down.
  ///
  /// This is useful for discrete actions that should repeatedly happen while a
  /// button is held, such as selecting menu items with the directional pad.
  pub fn is_pressed_repeated(&self, button: Button) -> bool {
    self.buttons[button as usize].repeated
  }
}

/// Sstate of an input button.
#[derive(Default, Debug)]
pub struct ButtonState {
  /// Time the button was first pressed or `None` if the button is not pressed.
  pub pressed_at: Option<f64>,
  /// Whether the press was repeated, which is `true` periodically while the
  /// button is held down.
  pub repeated: bool,
}

/// Sets up input for the given world.
pub fn setup(world: &mut World) {
  world.add_resource(Input::default());

  set_mapping(world, Mapping::default());
}
