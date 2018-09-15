// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use ggez::event::KeyCode;

#[derive(Default)]
pub struct Events {
  pub list: Vec<Event>,
}

#[derive(Debug)]
pub enum Event {
  Pressed(KeyCode),
  Released(KeyCode),
}
