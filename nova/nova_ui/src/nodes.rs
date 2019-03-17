// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod build;
mod child_nodes;
mod context;
mod hierarchy;
mod node;
mod read;
mod write;

pub use self::build::build;
pub use self::context::NodeContext;
pub use self::node::Node;
pub use self::read::ReadNodes;

pub(crate) use self::child_nodes::ChildNodes;
pub(crate) use self::hierarchy::NodeHierarchy;
pub(crate) use self::write::WriteNodes;

use nova_core::ecs;
use nova_core::engine::{Engine, Resources};

pub fn read(res: &Resources) -> ReadNodes {
  ecs::SystemData::fetch(res)
}

pub(crate) fn write(res: &Resources) -> WriteNodes {
  ecs::SystemData::fetch(res)
}

pub(crate) fn setup(engine: &mut Engine) {
  engine.res.entry().or_insert_with(NodeHierarchy::default);

  ecs::components::register::<Node>(&mut engine.res);

  build::setup(engine);
}
