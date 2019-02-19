// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod common;
pub mod spec;

mod context;
mod element;
mod hierarchy;
mod instance;
mod message;

pub use self::context::Context;
pub use self::element::{Element, ShouldRebuild};
pub use self::hierarchy::Hierarchy;
pub use self::message::{Message, MessageComposer, MessageQueue};
pub use self::spec::{spec, Spec};

use self::instance::Instance;
use crate::ecs;
use crate::engine::{self, Engine};

pub fn setup(engine: &mut Engine) {
  engine.resources_mut().insert(Hierarchy::new());
  engine.resources_mut().insert(MessageQueue::new());

  ecs::register::<hierarchy::Node>(engine.resources_mut());

  common::setup(engine);
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
