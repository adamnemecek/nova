// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::engine;

pub use specs::join::{Join, ParJoin};

pub use specs::shred::{DynamicSystemData, System, SystemData};
pub use specs::shred::{ReadExpect as ReadResource, WriteExpect as WriteResource};

pub use specs::storage;
pub use specs::storage::{BTreeStorage, DenseVecStorage, HashMapStorage, NullStorage, VecStorage};
pub use specs::storage::{ComponentEvent, FlaggedStorage};

pub use specs::world::Component;
pub use specs::world::{Builder as BuildEntity, EntityBuilder};
pub use specs::world::{EntitiesRes as Entities, Entity};

pub use specs::BitSet;
pub use specs::{ReadStorage as ReadComponents, WriteStorage as WriteComponents};

pub type ReadEntities<'a> = ReadResource<'a, Entities>;

pub fn register<T: Component>(res: &mut engine::Resources)
where
  T::Storage: Default,
{
  register_with_storage::<_, T>(res, Default::default);
}

pub fn register_with_storage<F, T>(res: &mut engine::Resources, storage: F)
where
  F: FnOnce() -> T::Storage,
  T: Component,
{
  res
    .entry()
    .or_insert_with(move || storage::MaskedStorage::<T>::new(storage()));

  res
    .fetch_mut::<engine::MetaTable<storage::AnyStorage>>()
    .register(&*res.fetch::<storage::MaskedStorage<T>>());
}

pub fn read_components<T>(res: &engine::Resources) -> ReadComponents<T>
where
  T: Component,
{
  storage::Storage::new(res.fetch(), res.fetch::<storage::MaskedStorage<T>>())
}

pub fn entities(res: &engine::Resources) -> engine::FetchResource<Entities> {
  res.fetch()
}
