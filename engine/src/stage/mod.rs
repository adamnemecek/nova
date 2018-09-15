// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use specs::prelude::*;

use prelude::*;

pub mod position;
pub mod renderer;

pub use self::position::Position;
pub use self::renderer::{Render, Renderer};

pub fn setup<'a, 'b>(core: &mut Core, _dispatch: &mut DispatcherBuilder<'a, 'b>) {
  core.world.register::<Position>();
  core.world.register::<Render>();
}
