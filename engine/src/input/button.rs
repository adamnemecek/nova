// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// One of the available input buttons.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Button {
  Up,
  Left,
  Down,
  Right,
}

/// Total number of available input buttons.
pub const BUTTON_COUNT: usize = Button::Right as usize + 1;
