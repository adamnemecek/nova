// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `engine` module contains core engine functionality such as setting up
//! the window and running ECS.
//!
//! The `window` module can create a platform-specific window for graphics and
//! input events. The state of the window is stored in the `Window` resource.
//!
//! The `context` module defines the `engine::Context`, the global state for the
//! engine. A context can be created from a `Window` or without one.
//!
//! The `init` module defines structures and functions that are used in the
//! initialization of an engine context. Until `engine::run` is called, the
//! context is still in init mode and can have new systems and processes
//! added.

pub mod init;

mod context;
mod extensions;
mod running;
mod window;

pub use self::context::*;
pub use self::extensions::*;
pub use self::running::*;
pub use self::window::*;

pub use specs::{
  Builder as EntityBuilderExt, Component, Entities, Entity, EntityBuilder, Join as StorageJoin,
  ParJoin as ParStorageJoin, ReadStorage, System, WriteStorage,
};

pub use specs::shred::{
  Fetch as FetchResource, FetchMut as FetchResourceMut, Read as ReadResource, Resource,
  Write as WriteResource,
};

/// Creates a new entity builder that will build an entity in the engine
/// context.
pub fn build_entity<'a>(ctx: &'a mut Context) -> EntityBuilder<'a> {
  ctx.world.create_entity()
}

/// Adds a resource to the engine context.
pub fn add_resource(ctx: &mut Context, resource: impl Resource) {
  ctx.world.add_resource(resource)
}

/// Fetches a resource from the engine context.
pub fn fetch_resource<'a, T: Resource + Send + 'a>(ctx: &'a Context) -> FetchResource<'a, T> {
  ctx.world.read_resource::<T>()
}

/// Mutably fetches a resource from the engine context.
pub fn fetch_resource_mut<'a, T: Resource + Send + 'a>(
  ctx: &'a Context,
) -> FetchResourceMut<'a, T> {
  ctx.world.write_resource::<T>()
}

/// Checks whether the engine context has a resource of type `T`.
pub fn has_resource<T: Resource + Send>(ctx: &Context) -> bool {
  ctx.world.res.has_value::<T>()
}

pub mod storages {
  pub use specs::storage::{
    BTreeStorage, DenseVecStorage, FlaggedStorage, HashMapStorage, NullStorage, ReadStorage,
    VecStorage, WriteStorage,
  };

  pub type FlaggedBTreeStorage<T> = FlaggedStorage<T, VecStorage<T>>;
}

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
