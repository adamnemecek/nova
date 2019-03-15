// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod axis;
mod button;
mod update;

pub use self::axis::GamepadAxis;
pub use self::button::GamepadButton;
pub use self::update::UpdateGamepad;

use nova_core::ecs;
use nova_core::engine::Resources;
use std::collections::HashMap;

pub type ReadGamepad<'a> = ecs::ReadResource<'a, Gamepad>;

type WriteGamepad<'a> = ecs::WriteResource<'a, Gamepad>;

#[derive(Debug, Default)]
pub struct Gamepad {
  buttons: HashMap<GamepadButton, f32>,
  axes: HashMap<GamepadAxis, f32>,
}

impl Gamepad {
  /// Gets the current value of a gamepad button.
  ///
  /// The value is between 0.0 and 1.0, where 1.0 is fully pressed and 0.0 is
  /// fully released.
  pub fn get_button(&self, button: GamepadButton) -> f32 {
    self.buttons.get(&button).cloned().unwrap_or_default()
  }

  /// Gets the current value of a gamepad axis.
  ///
  /// The value is between -1.0 and 1.0. The meaning depends on what the ax
  pub fn get_axis(&self, axis: GamepadAxis) -> f32 {
    self.axes.get(&axis).cloned().unwrap_or_default()
  }
}

pub fn read(res: &Resources) -> ReadGamepad {
  ecs::SystemData::fetch(res)
}
