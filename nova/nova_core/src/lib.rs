// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod component;
pub mod entity;
pub mod math;
pub mod time;

pub use self::component::{Component, Components, ComponentsMut};
pub use self::entity::{Entities, Entity};
pub use shred::{ReadExpect as Resource, Resource as ResourceLike, WriteExpect as ResourceMut};

/// Stores related state in resources, entities, and components.
///
/// Each instance can be thought of as a separate database for storing
/// application data and state.
#[derive(Default)]
pub struct Instance {
  world: specs::World,
}

impl Instance {
  /// Create a new, empty instance.
  pub fn new() -> Self {
    Self::default()
  }

  /// Gets an immutable reference to the resource of the type `R` in the
  /// instance.
  ///
  /// Panics if no such resource exists.
  pub fn resource<R: ResourceLike>(&self) -> Resource<R> {
    self.world.system_data()
  }

  /// Gets a mutable reference to the resource of the type `R` in the instance.
  ///
  /// Panics if no such resource exists.
  pub fn resource_mut<R: ResourceLike>(&self) -> ResourceMut<R> {
    self.world.system_data()
  }

  /// Adds a resource of type `R` to the instance.
  ///
  /// If a resource of type `R` already exists, it is dropped.
  pub fn put_resource<R: ResourceLike>(&mut self, value: R) {
    self.world.add_resource(value);
  }

  /// Returns an `Entities` struct for creating, deleting, and reading the
  /// entities in the instance.
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

  /// Registers a possible component type with the instance.
  pub fn register_component<C: Component>(&mut self)
  where
    C::Storage: Default,
  {
    self.world.register::<C>();
  }

  /// Returns a `Components<C>` struct for reading the components of type `C`
  /// for all entities in the instance.
  pub fn components<C: Component>(&self) -> Components<C> {
    self.world.system_data()
  }

  /// Returns a `ComponentsMut<C>` struct for reading and writing the components
  /// of type `C` for all entities in the instance.
  pub fn components_mut<C: Component>(&self) -> ComponentsMut<C> {
    self.world.system_data()
  }
}
