// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `ecs` module exposes a parallel ECS implementation based on [specs][1].
//!
//! [1]: https://github.com/slide-rs/specs
//!
pub use specs::join::{Join, ParJoin};

pub use specs::shred::{AsyncDispatcher, Dispatcher, DispatcherBuilder};
pub use specs::shred::{DynamicSystemData, System, SystemData};
pub use specs::shred::{Fetch as FetchResource, FetchMut as FetchResourceMut};
pub use specs::shred::{ReadExpect as ReadResource, WriteExpect as WriteResource};
pub use specs::shred::{Resource, Resources};

pub use specs::storage;
pub use specs::storage::{BTreeStorage, DenseVecStorage, HashMapStorage, NullStorage, VecStorage};
pub use specs::storage::{ComponentEvent, FlaggedStorage};

pub use specs::world::is_entity_alive;
pub use specs::world::Component;
pub use specs::world::{create_entity, delete_all_entities, delete_entities, delete_entity};
pub use specs::world::{entities, entities_mut};
pub use specs::world::{init, maintain};
pub use specs::world::{read_storage as read_components, write_storage as write_components};
pub use specs::world::{register, register_with_storage};
pub use specs::world::{Builder as BuildEntity, EntityBuilder};
pub use specs::world::{EntitiesRes as Entities, Entity};

pub use specs::BitSet;
pub use specs::{ReadStorage as ReadComponents, WriteStorage as WriteComponents};

pub type ReadEntities<'a> = ReadResource<'a, Entities>;

pub fn ensure_resource<R: Resource + Default>(res: &mut Resources) -> FetchResourceMut<R> {
  res.entry().or_insert_with(R::default)
}

pub fn setup<'a, S: System<'a>>(res: &'a mut Resources, system: &mut S) {
  system.setup(res);
}

pub fn run<'a, S: System<'a>>(res: &'a Resources, system: &mut S) {
  let data = S::SystemData::fetch(&system.accessor(), res);

  system.run(data);
}

pub fn exec<'a, D, F, R>(res: &'a Resources, mut func: F) -> R
where
  D: SystemData<'a>,
  F: FnMut(D) -> R,
{
  let data = D::fetch(res);

  func(data)
}

pub mod prelude {
  pub use crate::ecs::{
    self, AsyncDispatcher, BitSet, BuildEntity, Component, Dispatcher, DispatcherBuilder,
    DynamicSystemData, Entity, EntityBuilder, FetchResource, FetchResourceMut, Join, ParJoin,
    ReadComponents, ReadResource, Resource, Resources, System, SystemData, WriteComponents,
    WriteResource,
  };
}
