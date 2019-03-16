// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod derive;

use crate::engine;

pub use specs::join::{Join, ParJoin};
pub use specs::shred::{ReadExpect as ReadResource, WriteExpect as WriteResource};
pub use specs::shred::{System, SystemData};
pub use specs::storage;
pub use specs::storage::{BTreeStorage, DenseVecStorage, HashMapStorage, NullStorage, VecStorage};
pub use specs::storage::{ComponentEvent, FlaggedStorage};
pub use specs::world::Component;
pub use specs::world::{Builder as BuildEntity, EntityBuilder};
pub use specs::world::{EntitiesRes as Entities, Entity};
pub use specs::BitSet;
pub use specs::{ReadStorage as ReadComponents, WriteStorage as WriteComponents};
pub use self::entities::{ReadEntities, WriteEntities};

use crate::engine::Resources;

pub fn register<T: Component>(res: &mut Resources)
where
  T::Storage: Default,
{
  register_with_storage::<_, T>(res, Default::default);
}

pub fn register_with_storage<F, T>(res: &mut Resources, storage: F)
where
  F: FnOnce() -> T::Storage,
  T: Component,
{
  res
    .entry()
    .or_insert_with(move || storage::MaskedStorage::<T>::new(storage()));

  res
    .fetch_mut::<engine::resources::MetaTable<storage::AnyStorage>>()
    .register(&*res.fetch::<storage::MaskedStorage<T>>());
}

pub fn read_components<T>(res: &Resources) -> ReadComponents<T>
where
  T: Component,
{
  ReadComponents::fetch(res)
}

pub fn write_components<T>(res: &Resources) -> WriteComponents<T>
where
  T: Component,
{
  WriteComponents::fetch(res)
}

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
