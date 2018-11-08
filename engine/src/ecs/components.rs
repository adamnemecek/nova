// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use specs::Join as ComponentJoin;
pub use specs::ParJoin as ParComponentJoin;
pub use specs::{Component, ReadStorage, WriteStorage};

use super::{Context, Entity};

/// Adds storage for components of type `T` to the ECS context.
pub fn register_component<T>(ctx: &mut Context)
where
  T: Component,
  T::Storage: Default,
{
  ctx.world.register::<T>();
}

/// Fetches the storage for components of type `T` to the ECS context.
pub fn fetch_components<'a, T: Component>(ctx: &'a Context) -> ReadStorage<'a, T> {
  ctx.world.read_storage()
}

/// Mutably fetches the storage for components of type `T` to the ECS context.
pub fn fetch_components_mut<'a, T: Component>(ctx: &'a Context) -> WriteStorage<'a, T> {
  ctx.world.write_storage()
}

/// Fetches the component for the entity in the given ECS context and passes it
/// by mutable reference to the given `editor` function. Returns the result of
/// that function.
pub fn edit_component<'a, T: Component, R>(
  ctx: &'a Context,
  entity: Entity,
  editor: impl FnOnce(&mut T) -> R,
) -> R {
  let mut storage = fetch_components_mut(ctx);

  let component = storage
    .get_mut(entity)
    .expect("entity does not have that component");

  editor(component)
}
