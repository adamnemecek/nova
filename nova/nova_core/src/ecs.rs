// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod components;
pub mod derive;

pub use self::components::{Component, ReadComponents, WriteComponents};
pub use self::entities::{ReadEntities, WriteEntities};
pub use specs::join::{Join, ParJoin};
pub use specs::shred::{ReadExpect as ReadResource, WriteExpect as WriteResource};
pub use specs::shred::{System, SystemData};
pub use specs::storage;
pub use specs::storage::{BTreeStorage, DenseVecStorage, HashMapStorage, NullStorage, VecStorage};
pub use specs::storage::{ComponentEvent, FlaggedStorage};
pub use specs::world::{Builder as BuildEntity, EntityBuilder};
pub use specs::world::{EntitiesRes as Entities, Entity};
pub use specs::BitSet;

pub mod entities {
  use crate::ecs::{Entities, ReadResource, SystemData, WriteResource};
  use crate::engine::Resources;

  pub type ReadEntities<'a> = ReadResource<'a, Entities>;
  pub type WriteEntities<'a> = WriteResource<'a, Entities>;

  pub fn read(res: &Resources) -> ReadEntities {
    SystemData::fetch(res)
  }

  pub fn write(res: &Resources) -> WriteEntities {
    SystemData::fetch(res)
  }
}
