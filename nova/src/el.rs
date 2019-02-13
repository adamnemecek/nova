// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod node;

mod hierarchy;
mod instance;
mod mount;
mod prototype;

use self::hierarchy::Hierarchy;
use self::instance::InstanceBox;
use self::mount::Mount;
use self::prototype::Prototype;
use crate::ecs;
use crate::engine;
use std::fmt;
use std::ops::{Deref, DerefMut};

pub use self::hierarchy::BuildHierarchy;
pub use self::node::{node, ChildNodes, Node};

pub trait Element: PartialEq + Send + Sync + fmt::Debug {
  type State: Default + Send + Sync + fmt::Debug + 'static;

  fn on_awake(&self, _state: &mut Self::State) {}
  fn on_sleep(&self, _state: &mut Self::State) {}

  fn on_change(&self, _state: &mut Self::State) -> ShouldRebuild {
    ShouldRebuild(true)
  }

  fn build(&self, _state: &mut Self::State, _children: ChildNodes) -> Node {
    node::empty()
  }
}

pub trait StatelessElement: PartialEq + Send + Sync + fmt::Debug {
  fn on_awake(&self) {}
  fn on_sleep(&self) {}

  fn on_change(&self) -> ShouldRebuild {
    ShouldRebuild(true)
  }

  fn build(&self, _children: ChildNodes) -> Node {
    node::empty()
  }
}

impl<T: StatelessElement> Element for T {
  type State = ();

  fn on_awake(&self, _state: &mut Self::State) {
    self.on_awake();
  }

  fn on_sleep(&self, _state: &mut Self::State) {
    self.on_sleep();
  }

  fn on_change(&self, _state: &mut Self::State) -> ShouldRebuild {
    self.on_change()
  }

  fn build(&self, _state: &mut Self::State, children: ChildNodes) -> Node {
    self.build(children)
  }
}

pub trait PureElement: PartialEq + Send + Sync + fmt::Debug {
  fn build(&self, _children: ChildNodes) -> Node {
    node::empty()
  }
}

impl<T: PureElement> StatelessElement for T {
  fn build(&self, children: ChildNodes) -> Node {
    PureElement::build(self, children)
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

#[derive(Default)]
pub struct RebuildRequired;

impl ecs::Component for RebuildRequired {
  type Storage = ecs::NullStorage<Self>;
}

pub fn create<E: Element + 'static>(res: &engine::Resources, element: E) {
  let entities = res.fetch::<ecs::Entities>();

  let mut hierarchy = res.fetch_mut::<Hierarchy>();
  let mut mounts = ecs::write_components::<Mount>(res);
  let mut rebuild_required = ecs::write_components::<RebuildRequired>(res);

  hierarchy.roots.push(
    entities
      .build_entity()
      .with(Mount::new(InstanceBox::new(element)), &mut mounts)
      .with(RebuildRequired, &mut rebuild_required)
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
