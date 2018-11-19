// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use specs::Component;
pub use specs::{Join, ParJoin};
pub use specs::{ReadStorage as ReadComponents, WriteStorage as WriteComponents};

use crate::Engine;

pub fn read_components<T: Component>(engine: &Engine) -> ReadComponents<T> {
  engine.world.read_storage()
}

pub fn write_components<T: Component>(engine: &Engine) -> WriteComponents<T> {
  engine.world.write_storage()
}
