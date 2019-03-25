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

use nova_core::components;
use nova_core::engine::Engine;
use nova_core::resources::Resources;
use nova_core::systems::SystemData;

pub fn borrow(res: &Resources) -> ReadNodes {
  SystemData::fetch(res)
}

pub(crate) fn borrow_mut(res: &Resources) -> WriteNodes {
  SystemData::fetch(res)
}

pub(crate) fn set_up(engine: &mut Engine) {
  engine
    .resources
    .entry()
    .or_insert_with(NodeHierarchy::default);

  components::register::<Node>(&mut engine.resources);

  build::set_up(engine);
}
