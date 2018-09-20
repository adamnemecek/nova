// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

pub mod actors;
pub mod objects;

pub use self::objects::draw;

/// Sets up the given world for stage visuals.
pub fn setup<'a, 'b>(world: &mut World, systems: &mut DispatcherBuilder<'a, 'b>) {
  objects::setup(world, systems);
  actors::setup(world, systems);
}
