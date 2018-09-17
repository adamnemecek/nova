// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use specs::DispatcherBuilder;

use prelude::*;

pub mod buttons;
pub mod system;

pub use self::buttons::Button;
pub use self::system::InputSystem;

/// Resource that stores the current input state.
#[derive(Default, Debug)]
pub struct Input {
  /// State of all available input buttons.
  pub buttons: [ButtonState; buttons::COUNT],
}

impl Input {
  /// Returns `true` while the given `button` is held down.
  pub fn is_pressed(&self, button: Button) -> bool {
    self.buttons[button as usize].pressed_time.is_some()
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
  /// `None`.
  pub pressed_time: Option<f64>,
  /// Whether the press was repeated, which is `true` periodically while the
  /// button is held down.
  pub repeated: bool,
}

/// Sets up input components, resources, and systems.
pub fn setup<'a, 'b>(core: &mut Core, dispatch: &mut DispatcherBuilder<'a, 'b>) {
  core.world.add_resource(Input::default());

  dispatch.add(InputSystem::default(), "input::InputSystem", &[]);
}
