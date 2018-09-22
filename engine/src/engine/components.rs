// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

pub use specs::{
  storage::BTreeStorage, Component, DenseVecStorage, FlaggedStorage, HashMapStorage, NullStorage,
  ReadStorage, WriteStorage,
};

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
