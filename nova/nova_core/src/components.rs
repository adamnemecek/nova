// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod storages;

pub use self::storages::{BTreeStorage, DenseVecStorage, HashMapStorage, NullStorage, VecStorage};
pub use self::storages::{ComponentEvent, FlaggedStorage};
pub use specs::join::{Join, ParJoin};
pub use specs::{BitSet, Component};
pub use specs::{ReadStorage as ReadComponents, WriteStorage as WriteComponents};

use self::storages::{AnyStorage, MaskedStorage};
use crate::entities::Entity;
use crate::resources::{ResourceMetaTable, Resources};
use crate::systems::SystemData;

pub fn set_up(resources: &mut Resources) {
  resources
    .entry()
    .or_insert_with(ResourceMetaTable::<AnyStorage>::default);
}

pub fn borrow<T: Component>(res: &Resources) -> ReadComponents<T> {
  SystemData::fetch(res)
}

pub fn borrow_mut<T: Component>(res: &Resources) -> WriteComponents<T> {
  SystemData::fetch(res)
}

pub fn register<T>(res: &mut Resources)
where
  T: Component,
  T::Storage: Default,
{
  register_with_storage::<T, _>(res, Default::default);
}

pub fn register_with_storage<T, F>(res: &mut Resources, storage: F)
where
  T: Component,
  F: FnOnce() -> T::Storage,
{
  res
    .entry()
    .or_insert_with(move || MaskedStorage::<T>::new(storage()));

  {
    let table = res.try_fetch_mut::<ResourceMetaTable<AnyStorage>>();

    if let Some(mut table) = table {
      table.register(&*res.fetch::<MaskedStorage<T>>());
      return;
    }
  }

  let mut table = ResourceMetaTable::<AnyStorage>::new();

  table.register(&*res.fetch::<MaskedStorage<T>>());
  res.insert(table);
}

pub fn delete_all_for_entities(resources: &Resources, entities: &[Entity]) {
  let storages = resources.fetch_mut::<ResourceMetaTable<AnyStorage>>();

  for storage in storages.iter_mut(resources) {
    storage.drop(entities);
  }
}
