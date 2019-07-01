// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use specs::Entity;

use super::*;

/// Provides access to the entities in an instance.
#[derive(SystemData)]
pub struct Entities<'a>(specs::Entities<'a>);

impl<'a> Entities<'a> {
  /// Creates a new entity.
  ///
  /// After creating one or more entities, the `Instance::commit_entities()`
  /// function should be called.
  pub fn create(&self) -> Entity {
    self.0.create()
  }

  /// Deletes the given entity, returning `true` if the entity is alive.
  ///
  /// A deleted entity is still considered alive until the
  /// `Instance::commit_entities()` function is called.
  pub fn delete(&self, entity: Entity) -> bool {
    self.0.delete(entity).is_ok()
  }

  /// Returns `true` if the given entity is alive.
  ///
  /// A deleted entity is considered alive until the
  /// `Instance::commit_entities()` function is called.
  pub fn is_alive(&self, entity: Entity) -> bool {
    self.0.is_alive(entity)
  }
}

impl<'a, 'b> Join for &'a Entities<'b> {
  type Type = Entity;
  type Value = &'a specs::world::EntitiesRes;
  type Mask = BitSetOr<&'a BitSet, &'a AtomicBitSet>;

  unsafe fn open(self) -> (Self::Mask, Self::Value) {
    self.0.open()
  }

  unsafe fn get(this: &mut Self::Value, idx: specs::world::Index) -> Entity {
    Self::Value::get(this, idx)
  }
}
