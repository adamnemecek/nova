// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod controls;
pub mod gamepad;
pub mod keyboard;
pub mod mouse;

use nova_core::engine::Engine;

pub fn set_up(engine: &mut Engine) {
  gamepad::set_up(engine);
  keyboard::set_up(engine);
  mouse::set_up(engine);

  controls::set_up(engine);
}
