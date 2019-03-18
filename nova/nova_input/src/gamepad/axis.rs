// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum GamepadAxis {
  LeftStickX,
  LeftStickY,
  RightStickX,
  RightStickY,
}

impl GamepadAxis {
  pub(crate) fn from_gilrs(axis: gilrs::Axis) -> Option<Self> {
    Some(match axis {
      gilrs::Axis::LeftStickX => GamepadAxis::LeftStickX,
      gilrs::Axis::LeftStickY => GamepadAxis::LeftStickY,
      gilrs::Axis::RightStickX => GamepadAxis::RightStickX,
      gilrs::Axis::RightStickY => GamepadAxis::RightStickY,

      _ => return None,
    })
  }
}
