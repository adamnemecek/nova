// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Character {
  pub speed: f32,
  pub state: State,
}

#[derive(Debug)]
pub enum State {
  Idle,
  Walking,
  Running,
}

impl Default for State {
  fn default() -> Self {
    State::Idle
  }
}
