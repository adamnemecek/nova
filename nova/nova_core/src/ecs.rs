// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod components;
pub mod derive;
pub mod entities;
pub mod resources;
pub mod system;

pub use self::components::{Component, Join, ParJoin, ReadComponents, WriteComponents};
pub use self::entities::{Entities, Entity, ReadEntities, WriteEntities};
pub use self::resources::{ReadResource, Resource, Resources, WriteResource};
pub use self::system::{System, SystemData};
use crate::engine::Engine;
pub use specs::storage;
pub use specs::storage::{BTreeStorage, DenseVecStorage, HashMapStorage, NullStorage, VecStorage};
pub use specs::storage::{ComponentEvent, FlaggedStorage};
pub use specs::BitSet;

pub(crate) fn setup(engine: &mut Engine) {
  engine.resources.entry().or_insert_with(Entities::default);

  engine
    .resources
    .entry()
    .or_insert_with(resources::MetaTable::<storage::AnyStorage>::default);
}
