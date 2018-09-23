// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `input` module abstracts keyboard input, mapping it to virtual input
//! buttons.
//!
//! This module creates a `Mapping` resource which maps keyboard `KeyCode` to
//! virtual input `Button`. The resource can be modified to change key bindings
//! and can be loaded from and saved to a file.
//!
//! This module also creates an `Input` resource which stores the state of
//! virtual input buttons and an `Updater` system which updates that resource
//! using input from the `engine::Window`. If the engine context has no window,
//! the system will run but no input will occur.

use crate::prelude::*;

mod button;
mod mapping;
mod updater;

pub use self::button::*;
pub use self::mapping::*;
pub use self::updater::*;

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

pub fn init(ctx: &mut engine::Context) {
  engine::add_resource(ctx, Input::default());

  set_mapping(ctx, Mapping::default());

  engine::init::add_system_early(ctx, Updater, "input::Updater", &[]);
}
