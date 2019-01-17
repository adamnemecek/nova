// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `ecs` module exposes a parallel ECS implementation based on [specs][1].
//!
//! [1]: https://github.com/slide-rs/specs
//!
pub use specs::join::{Join, ParJoin};

pub use specs::shred::{par, seq, Par, ParSeq as Dispatcher, RunWithPool as Dispatchable, Seq};
pub use specs::shred::{DynamicSystemData, RunNow as Runnable, System, SystemData};
pub use specs::shred::{Fetch, FetchMut};
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

use crate::thread::ThreadPool;
use std::borrow::BorrowMut;

pub type ReadEntities<'a> = ReadResource<'a, Entities>;

pub fn setup<D: for<'a> Dispatchable<'a>>(res: &mut Resources, dispatchable: &mut D) {
  dispatchable.setup(res);
}

pub fn run<R: for<'a> Runnable<'a>>(res: &Resources, system: &mut R) {
  system.run_now(res);
}

pub fn dispatch<D: for<'a> Dispatchable<'a>>(
  res: &mut Resources,
  dispatchable: &mut D,
  pool: &ThreadPool,
) {
  dispatchable.run(res, pool);
}

pub trait ResourceFetch: Resource + Sized {
  fn fetch(res: &Resources) -> Fetch<Self> {
    res.fetch()
  }

  fn fetch_mut(res: &Resources) -> FetchMut<Self> {
    res.fetch_mut()
  }
}

impl<T: Resource + Sized> ResourceFetch for T {}
