// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
pub(crate) use graphics::*;

pub mod actors;
pub mod objects;

mod camera;

pub use self::camera::*;

/// Initializes stage graphics for the given engine context.
pub fn init(ctx: &mut engine::Context) {
  engine::add_resource(ctx, Camera::default());

  objects::init(ctx);
  actors::init(ctx);
}
