// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A standalone ECS context containing resources, entities, and components.
#[derive(Default)]
pub struct Context {
  world: specs::World,
}

impl Context {
  /// Create a new, empty ECS context.
  pub fn new() -> Self {
    Self::default()
  }

  /// Gets an immutable reference to the resource of the type `R`.
  ///
  /// Panics if no such resource exists.
  pub fn resource<R: ResourceLike>(&self) -> Resource<R> {
    self.world.system_data()
  }

  /// Gets a mutable reference to the resource of the type `R`.
  ///
  /// Panics if no such resource exists.
  pub fn resource_mut<R: ResourceLike>(&self) -> ResourceMut<R> {
    self.world.system_data()
  }

  /// Sets the available resource of type `R`.
  ///
  /// If a resource of type `R` already exists, the old value dropped.
  pub fn put_resource<R: ResourceLike>(&mut self, value: R) {
    self.world.add_resource(value);
  }

  /// Returns an `Entities` struct for creating, deleting, and listing entities.
  pub fn entities(&self) -> Entities {
    self.world.system_data()
  }

  /// Finalizes the creation and deletion of any entities created or deleted
  /// since the previous call to this function.
  ///
  /// A deleted entity is still considered alive until this function is called.
  pub fn commit_entities(&mut self) {
    self.world.maintain();
  }

  /// Registers a possible component type.
  pub fn register_component<C: Component>(&mut self)
  where
    C::Storage: Default,
  {
    self.world.register::<C>();
  }

  /// Returns a `Components<C>` struct for reading the components of type `C`
  /// for all entities.
  pub fn components<C: Component>(&self) -> Components<C> {
    self.world.system_data()
  }

  /// Returns a `ComponentsMut<C>` struct for reading and writing the components
  /// of type `C` for all entities.
  pub fn components_mut<C: Component>(&self) -> ComponentsMut<C> {
    self.world.system_data()
  }
}
