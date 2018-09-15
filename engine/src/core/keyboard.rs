// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

pub type KeyCode = ggez::event::KeyCode;

#[derive(Default)]
pub struct Events {
  pub list: Vec<Event>,
}

pub enum Event {
  Pressed(KeyCode),
  Released(KeyCode),
}
