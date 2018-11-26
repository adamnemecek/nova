// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `ecs` module exposes a parallel ECS implementation based on [specs][1].
//!
//! [1]: https://github.com/slide-rs/specs

pub mod derive;
pub mod systems;

pub mod storages {
  pub use specs::storage::{
    BTreeStorage, DenseVecStorage, FlaggedStorage, HashMapStorage, NullStorage, ReadStorage,
    VecStorage, WriteStorage,
  };
}

pub use self::storages::*;
pub use self::systems::*;

pub use specs::Component;
pub use specs::{Join, ParJoin};
pub use specs::{ReadStorage as ReadComponents, WriteStorage as WriteComponents};

pub use specs::world::EntitiesRes as Entities;
pub use specs::Builder as BuildEntity;
pub use specs::Entities as ReadEntities;
pub use specs::{Entity, EntityBuilder};

pub use specs::shred::Resource;
pub use specs::shred::{Fetch as FetchResource, FetchMut as FetchResourceMut};
pub use specs::{ReadExpect as ReadResource, WriteExpect as WriteResource};
