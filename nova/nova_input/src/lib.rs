// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod gamepad;
pub mod keyboard;
pub mod mouse;

use nova_core::engine::Engine;

pub fn setup(engine: &mut Engine) {
  gamepad::setup(engine);
  keyboard::setup(engine);
  mouse::setup(engine);
}
