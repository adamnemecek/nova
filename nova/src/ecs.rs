// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use specs::join::{Join, ParJoin};
pub use specs::{self, shred};

pub use specs::shred::{par, seq, Par, ParSeq as Dispatcher, RunWithPool as Dispatchable, Seq};
pub use specs::shred::{DynamicSystemData, RunNow as Runnable, System, SystemData};
pub use specs::shred::{Fetch, FetchMut};
pub use specs::shred::{ReadExpect as ReadResource, WriteExpect as WriteResource};
pub use specs::shred::{Resource, Resources};

pub use specs::storage;
pub use specs::storage::{BTreeStorage, DenseVecStorage, HashMapStorage, NullStorage, VecStorage};
pub use specs::storage::{ComponentEvent, FlaggedStorage};

pub use specs::world::Component;
pub use specs::world::{Builder as BuildEntity, EntityBuilder};
pub use specs::world::{EntitiesRes as Entities, Entity};

pub use specs::BitSet;
pub use specs::{ReadStorage as ReadComponents, WriteStorage as WriteComponents};

pub type ReadEntities<'a> = ReadResource<'a, Entities>;

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
    .fetch_mut::<shred::MetaTable<storage::AnyStorage>>()
    .register(&*res.fetch::<storage::MaskedStorage<T>>());
}

pub fn read_components<T>(res: &Resources) -> ReadComponents<T>
where
  T: Component,
{
  storage::Storage::new(res.fetch(), res.fetch::<storage::MaskedStorage<T>>())
}

pub fn entities(res: &Resources) -> Fetch<Entities> {
  res.fetch()
}
