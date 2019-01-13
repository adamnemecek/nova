// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `ecs` module exposes a parallel ECS implementation based on [specs][1].
//!
//! [1]: https://github.com/slide-rs/specs

mod context;
pub mod derive;
pub mod systems;

pub mod storages {
  pub use specs::storage::{
    BTreeStorage, DenseVecStorage, FlaggedStorage, HashMapStorage, NullStorage, ReadStorage,
    VecStorage, WriteStorage,
  };
}

pub mod prelude {
  pub use super::derive::*;
  pub use super::{
    Component, Join, ParJoin, ReadComponents, ReadEntities, ReadResource, Resource, System,
    SystemData, WriteComponents, WriteResource,
  };
}

pub use self::context::*;
pub use self::storages::*;
pub use self::systems::*;
use super::EngineHandle;
pub use specs::shred::Resource;
pub use specs::shred::{Fetch as FetchResource, FetchMut as FetchResourceMut};
pub use specs::world::EntitiesRes as Entities;
pub use specs::Builder as BuildEntity;
pub use specs::Component;
pub use specs::Entities as ReadEntities;
pub use specs::{Entity, EntityBuilder};
pub use specs::{Join, ParJoin};
pub use specs::{ReadExpect as ReadResource, WriteExpect as WriteResource};
pub use specs::{ReadStorage as ReadComponents, WriteStorage as WriteComponents};

pub fn maintain(engine: &EngineHandle) {
  engine.execute_mut(Context::maintain);
}
