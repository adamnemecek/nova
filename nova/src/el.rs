// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod node;

mod context;
mod hierarchy;
mod instance;
mod message;
mod mount;
mod prototype;

use self::context::MountContext;
use self::hierarchy::Hierarchy;
use self::instance::InstanceBox;
use self::mount::Mount;
use self::prototype::Prototype;
use crate::ecs;
use crate::engine;
use std::fmt;
use std::ops::{Deref, DerefMut};

pub use self::context::Context;
pub use self::hierarchy::BuildHierarchy;
pub use self::message::{DeliverMessages, Message, MessageComposer};
pub use self::node::{node, ChildNodes, Node};

pub trait Element: PartialEq + Send + Sync + fmt::Debug + Sized {
  type State: Default + Send + Sync + fmt::Debug + 'static;
  type Message: message::Payload;

  fn on_awake(&self, _ctx: Context<Self>) {}
  fn on_sleep(&self, _ctx: Context<Self>) {}

  fn on_change(&self, _old: Self, _ctx: Context<Self>) -> ShouldRebuild {
    ShouldRebuild(true)
  }

  fn on_message(&self, _msg: Self::Message, _ctx: Context<Self>) -> ShouldRebuild {
    ShouldRebuild(false)
  }

  fn build(&self, ctx: Context<Self>) -> Node {
    ctx.children.into()
  }
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
pub struct ShouldRebuild(pub bool);

impl Deref for ShouldRebuild {
  type Target = bool;

  fn deref(&self) -> &bool {
    &self.0
  }
}

impl DerefMut for ShouldRebuild {
  fn deref_mut(&mut self) -> &mut bool {
    &mut self.0
  }
}

pub fn create<E: Element + 'static>(res: &engine::Resources, element: E) {
  let entities = res.fetch::<ecs::Entities>();

  let mut hierarchy = res.fetch_mut::<Hierarchy>();
  let mut mounts = ecs::write_components::<Mount>(res);

  hierarchy.roots.push(
    entities
      .build_entity()
      .with(Mount::new(InstanceBox::new(element)), &mut mounts)
      .build(),
  );
}

pub fn print_all(res: &engine::Resources) {
  let hierarchy = res.fetch::<Hierarchy>();
  let mounts = ecs::read_components::<Mount>(res);

  for entity in hierarchy.sorted.iter().cloned() {
    print!("\n{}: ", entity.id());

    if let Some(mount) = mounts.get(entity) {
      print!("{:#?}, ", mount.instance);

      print!(
        "node_children: {:?}, ",
        mount
          .node_children
          .entities
          .iter()
          .map(|e| e.id())
          .collect::<Vec<_>>()
      );

      println!(
        "real_children: {:?}",
        mount
          .real_children
          .entities
          .iter()
          .map(|e| e.id())
          .collect::<Vec<_>>()
      );
    }
  }
}
