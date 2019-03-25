// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod axis;
mod button;
mod update;

pub use self::axis::GamepadAxis;
pub use self::button::GamepadButton;
pub use self::update::UpdateGamepad;

use nova_core::collections::HashMap;
use nova_core::engine::Engine;
use nova_core::events::EventChannel;
use nova_core::resources::{self, ReadResource, Resources, WriteResource};
use std::f32;

pub type ReadGamepad<'a> = ReadResource<'a, Gamepad>;
pub type WriteGamepad<'a> = WriteResource<'a, Gamepad>;

#[derive(Default)]
pub struct Gamepad {
  pub events: EventChannel<GamepadEvent>,
  buttons: HashMap<GamepadButton, f32>,
  axes: HashMap<GamepadAxis, f32>,
}

impl Gamepad {
  pub fn borrow(res: &Resources) -> ReadGamepad {
    resources::borrow(res)
  }

  pub fn borrow_mut(res: &Resources) -> WriteGamepad {
    resources::borrow_mut(res)
  }

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

  fn set_button(&mut self, button: GamepadButton, value: f32) {
    let prev = self.buttons.insert(button, value).unwrap_or_default();

    if (prev - value).abs() > f32::EPSILON {
      self
        .events
        .single_write(GamepadEvent::ButtonChanged { button, value });
    }
  }

  fn set_axis(&mut self, axis: GamepadAxis, value: f32) {
    let prev = self.axes.insert(axis, value).unwrap_or_default();

    if (prev - value).abs() > f32::EPSILON {
      self
        .events
        .single_write(GamepadEvent::AxisChanged { axis, value });
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum GamepadEvent {
  ButtonChanged { button: GamepadButton, value: f32 },
  AxisChanged { axis: GamepadAxis, value: f32 },
}

pub fn set_up(engine: &mut Engine) {
  engine.resources.insert(Gamepad::default());

  update::set_up(engine);
}
