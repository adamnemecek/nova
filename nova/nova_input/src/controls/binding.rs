// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::gamepad::{GamepadAxis, GamepadButton};
use crate::keyboard::KeyCode;
use crate::mouse::MouseButton;
use serde_derive::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ControlBinding {
  Key(KeyCode),
  MouseButton(MouseButton),
  GamepadButton(GamepadButton),
  GamepadAxis(GamepadAxis),
}
