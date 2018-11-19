// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use specs::world::EntitiesRes as Entities;
pub use specs::Builder as BuildEntity;
pub use specs::Entities as ReadEntities;
pub use specs::{Entity, EntityBuilder};

use crate::Engine;

/// Creates a new entity in the given engine instance using the returned
/// `EntityBuilder`.
pub fn new_entity(engine: &mut Engine) -> EntityBuilder {
  engine.world.create_entity()
}
