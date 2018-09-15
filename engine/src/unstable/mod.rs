// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

pub mod character;
pub mod movement;

pub use self::character::Character;

pub fn setup<'a, 'b>(core: &mut Core, dispatch: &mut DispatcherBuilder<'a, 'b>) {
  core.world.register::<movement::Controlled>();
  core.world.register::<Character>();

  dispatch.add(movement::Controller, "unstable::movement::Controller", &[]);
}
