// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Context, Entity};

pub mod storages {
  pub use specs::storage::{
    BTreeStorage, DenseVecStorage, FlaggedStorage, HashMapStorage, NullStorage, ReadStorage,
    VecStorage, WriteStorage,
  };

  pub type FlaggedBTreeStorage<T> = FlaggedStorage<T, VecStorage<T>>;
}

pub use self::storages::*;
pub use specs::Join as ComponentJoin;
pub use specs::ParJoin as ParComponentJoin;
pub use specs::{Component, ReadStorage, WriteStorage};

/// Adds storage for components of type `T` to the engine context.
pub fn add_storage<T>(ctx: &mut Context)
where
  T: Component,
  T::Storage: Default,
{
  ctx.world.register::<T>();
}

/// Fetches the storage for components of type `T` to the engine context.
pub fn fetch_storage<'a, T: Component>(ctx: &'a Context) -> ReadStorage<'a, T> {
  ctx.world.read_storage::<T>()
}

/// Mutably fetches the storage for components of type `T` to the engine
/// context.
pub fn fetch_storage_mut<'a, T: Component>(ctx: &'a Context) -> WriteStorage<'a, T> {
  ctx.world.write_storage::<T>()
}

/// Fetches the component for the entity in the given engine context and passes
/// it by mutable reference to the given `editor` function. Returns the result
/// of that function.
pub fn edit_component<'a, T: Component, R>(
  ctx: &'a Context,
  entity: Entity,
  editor: impl FnOnce(&mut T) -> R,
) -> R {
  let mut storage = fetch_storage_mut::<T>(ctx);

  let component = storage
    .get_mut(entity)
    .expect("entity does not have that component");

  editor(component)
}
