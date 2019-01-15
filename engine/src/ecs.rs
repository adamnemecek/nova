// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `ecs` module exposes a parallel ECS implementation based on [specs][1].
//!
//! [1]: https://github.com/slide-rs/specs

pub use specs::shred::{
  Accessor, AsyncDispatcher, Dispatcher, DispatcherBuilder, DynamicSystemData,
  Fetch as FetchResource, FetchMut as FetchResourceMut, MetaTable, ReadExpect as ReadResource,
  Resource, ResourceId, Resources, RunNow, System, SystemData, WriteExpect as WriteResource,
};

pub use specs::{
  Component, Entities as ReadEntities, Join, ParJoin, ReadStorage, World, WriteStorage,
};

pub use specs::world::EntitiesRes as Entities;

pub mod derive {
  pub use super::storage::*;
  pub use super::{Component, SystemData};
  pub use specs_derive::*;
}

pub mod prelude {
  pub use super::derive::*;

  pub use super::{
    Component, FetchResource, FetchResourceMut, Join, ParJoin, ReadEntities, ReadResource,
    ReadStorage, Resource, Resources, System, SystemData, World, WriteResource, WriteStorage,
  };

  pub use crate::ecs;
}

pub mod storage {
  pub use specs::storage::*;
}

pub fn exec<'a, D, F, R>(res: &'a Resources, mut func: F) -> R
where
  D: SystemData<'a>,
  F: FnMut(D) -> R,
{
  let data = D::fetch(res);

  func(data)
}

pub fn run<'a, S: System<'a>>(res: &'a Resources, system: &mut S) {
  let data = S::SystemData::fetch(&system.accessor(), res);

  system.run(data);
}

pub fn run_once<'a, S: System<'a>>(res: &'a Resources, mut system: S) {
  run(res, &mut system);
}
