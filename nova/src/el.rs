// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod spec;

mod context;
mod element;
mod hierarchy;
mod instance;
mod message;

use self::instance::InstanceBox;
use crate::ecs;
use crate::engine;

pub use self::context::Context;
pub use self::element::{Element, ShouldRebuild};
pub use self::hierarchy::Hierarchy;
pub use self::message::{Message, MessageComposer};
pub use self::spec::{spec, Spec};

pub fn setup(res: &mut engine::Resources) {
  ecs::register::<hierarchy::Node>(res);

  res.entry().or_insert_with(Hierarchy::new);
}

pub fn print_all(res: &engine::Resources) {
  let hierarchy = res.fetch::<Hierarchy>();
  let nodes = ecs::read_components::<hierarchy::Node>(res);

  for entity in hierarchy.sorted.iter().cloned() {
    print!("\n{}: ", entity.id());

    if let Some(node) = nodes.get(entity) {
      print!("{:#?}, ", node.instance);

      print!(
        "spec_children: {:?}, ",
        node
          .spec_children
          .entities
          .iter()
          .map(|e| e.id())
          .collect::<Vec<_>>()
      );

      println!(
        "real_children: {:?}",
        node
          .real_children
          .entities
          .iter()
          .map(|e| e.id())
          .collect::<Vec<_>>()
      );
    }
  }
}
