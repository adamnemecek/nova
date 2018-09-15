// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

pub mod buttons;
pub mod updater;

pub use self::buttons::Button;
pub use self::updater::InputUpdater;

#[derive(Default, Debug)]
pub struct Input {
  pub buttons: [ButtonState; buttons::COUNT],
}

#[derive(Default, Debug)]
pub struct ButtonState {
  pressed_time: Option<f64>,
  repeated: bool,
}

pub fn setup(core: &mut Core) {
  core.world.add_resource(Input::default());
}
