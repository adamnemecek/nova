// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use specs::join::{Join, ParJoin};
pub use specs::Component;
pub use specs::{ReadStorage as ReadComponents, WriteStorage as WriteComponents};

use crate::ecs::resources::{MetaTable, Resources};
use crate::ecs::storage::{AnyStorage, MaskedStorage};
use crate::ecs::SystemData;

pub fn read<T: Component>(res: &Resources) -> ReadComponents<T> {
  SystemData::fetch(res)
}

pub fn write<T: Component>(res: &Resources) -> WriteComponents<T> {
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
    let table = res.try_fetch_mut::<MetaTable<AnyStorage>>();

    if let Some(mut table) = table {
      table.register(&*res.fetch::<MaskedStorage<T>>());
      return;
    }
  }

  let mut table = MetaTable::<AnyStorage>::new();

  table.register(&*res.fetch::<MaskedStorage<T>>());
  res.insert(table);
}
