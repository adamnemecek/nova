// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use specs::DispatcherBuilder;

use prelude::*;

pub mod buttons;
pub mod state;
pub mod updater;

pub use self::buttons::Button;
pub use self::state::State;
pub use self::updater::Updater;

pub fn setup<'a, 'b>(core: &mut Core, builder: &mut DispatcherBuilder<'a, 'b>) {
  core.world.add_resource(State::default());
  builder.add(Updater::default(), "input::Updater", &[]);
}
