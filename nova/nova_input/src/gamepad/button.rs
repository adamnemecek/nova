// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GamepadButton {
  DPadUp,
  DPadDown,
  DPadLeft,
  DPadRight,
  North,
  South,
  West,
  East,
  Start,
  Select,
  LeftTrigger,
  RightTrigger,
  LeftTrigger2,
  RightTrigger2,
  LeftStick,
  RightStick,
}

impl GamepadButton {
  pub(crate) fn from_gilrs(button: gilrs::Button) -> Option<Self> {
    Some(match button {
      gilrs::Button::DPadUp => GamepadButton::DPadUp,
      gilrs::Button::DPadDown => GamepadButton::DPadDown,
      gilrs::Button::DPadLeft => GamepadButton::DPadLeft,
      gilrs::Button::DPadRight => GamepadButton::DPadRight,
      gilrs::Button::North => GamepadButton::North,
      gilrs::Button::South => GamepadButton::South,
      gilrs::Button::West => GamepadButton::West,
      gilrs::Button::East => GamepadButton::East,
      gilrs::Button::Start => GamepadButton::Start,
      gilrs::Button::Select => GamepadButton::Select,
      gilrs::Button::LeftTrigger => GamepadButton::LeftTrigger,
      gilrs::Button::RightTrigger => GamepadButton::RightTrigger,
      gilrs::Button::LeftTrigger2 => GamepadButton::LeftTrigger2,
      gilrs::Button::RightTrigger2 => GamepadButton::RightTrigger2,
      gilrs::Button::LeftThumb => GamepadButton::LeftStick,
      gilrs::Button::RightThumb => GamepadButton::RightStick,

      _ => return None,
    })
  }
}
