// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{Entity, Component, Join};
use hibitset::BitSet;
use shred_derive::*;
use specs::storage::UnprotectedStorage;

/// Provides read-only access to the components of type `C` in an instance.
#[derive(SystemData)]
pub struct Components<'a, C: Component>(specs::ReadStorage<'a, C>);

impl<'a, C: Component> Components<'a, C> {
  /// Gets an immutable reference to the component for the given entity if it
  /// exists.
  pub fn get(&self, entity: Entity) -> Option<&C> {
    self.0.get(entity)
  }
}

impl<'a, C: Component> Join for &'a Components<'a, C> {
  type Type = &'a C;
  type Value = &'a C::Storage;
  type Mask = &'a BitSet;

  unsafe fn open(self) -> (Self::Mask, Self::Value) {
    self.0.open()
  }

  unsafe fn get(value: &mut Self::Value, index: specs::world::Index) -> Self::Type {
    value.get(index)
  }
}

/// Provides read and write access to the components of type `C` in an instance.
#[derive(SystemData)]
pub struct ComponentsMut<'a, C: Component>(specs::WriteStorage<'a, C>);

impl<'a, C: Component> ComponentsMut<'a, C> {
  /// Gets an immutable reference to the component for the given entity if it
  /// exists.
  pub fn get(&self, entity: Entity) -> Option<&C> {
    self.0.get(entity)
  }

  /// Gets a mutable reference to the component for the given entity if it
  /// exists.
  pub fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
    self.0.get_mut(entity)
  }

  /// Sets the component for the given entity, returning the previous value if
  /// it already existed.
  ///
  /// If the given entity is not alive, this function has no effect and the
  /// given component is immediately dropped.
  pub fn insert(&mut self, entity: Entity, value: C) -> Option<C> {
    match self.0.insert(entity, value) {
      Ok(old @ Some(_)) => old,
      _ => None,
    }
  }

  /// Removes the component for the given entity, returning the value if it
  /// existed.
  pub fn remove(&mut self, entity: Entity) -> Option<C> {
    self.0.remove(entity)
  }
}

impl<'a, 'b, C: Component> Join for &'a mut ComponentsMut<'b, C> {
  type Type = &'a mut C;
  type Value = &'a mut C::Storage;
  type Mask = &'a BitSet;

  unsafe fn open(self) -> (Self::Mask, Self::Value) {
    (&mut self.0).open()
  }

  unsafe fn get(value: &mut Self::Value, index: specs::world::Index) -> Self::Type {
    let value: *mut Self::Value = value as *mut Self::Value;

    (*value).get_mut(index)
  }
}
