// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use specs::Component;
pub use specs::{ReadStorage as ReadComponents, WriteStorage as WriteComponents};

use crate::ecs::SystemData;
use crate::engine::Resources;

pub fn read<T: Component>(res: &Resources) -> ReadComponents<T> {
  SystemData::fetch(res)
}

pub fn write<T: Component>(res: &Resources) -> WriteComponents<T> {
  SystemData::fetch(res)
}
