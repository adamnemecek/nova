// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use specs::world::Builder as BuildEntity;
pub use specs::world::EntitiesRes as Entities;
pub use specs::world::{Entity, EntityBuilder};

use crate::ecs::storage::AnyStorage;
use crate::ecs::{ReadResource, SystemData, WriteResource};
use crate::engine::resources::{MetaTable, Resources};

pub type ReadEntities<'a> = ReadResource<'a, Entities>;
pub type WriteEntities<'a> = WriteResource<'a, Entities>;

pub fn read(res: &Resources) -> ReadEntities {
  SystemData::fetch(res)
}

pub fn write(res: &Resources) -> WriteEntities {
  SystemData::fetch(res)
}

pub(crate) fn maintain(res: &mut Resources, buffer: &mut Vec<Entity>) {
  write(res).merge_deleted(buffer);

  if buffer.is_empty() {
    return;
  }

  for storage in res.fetch_mut::<MetaTable<AnyStorage>>().iter_mut(res) {
    storage.drop(buffer);
  }
}
