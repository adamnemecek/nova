// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
pub(crate) use graphics::*;

pub mod actors;
pub mod objects;

pub use self::objects::draw;

mod camera;

pub use self::camera::*;

/// Sets up the given world for stage visuals.
pub fn setup<'a, 'b>(world: &mut World, systems: &mut DispatcherBuilder<'a, 'b>) {
  world.add_resource(Camera::default());

  objects::setup(world, systems);
  actors::setup(world, systems);
}
