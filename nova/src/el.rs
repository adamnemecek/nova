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

pub use self::hierarchy::BuildHierarchy;
pub use self::node::{node, Node};

pub trait Element: Send + Sync + fmt::Debug {
  type Props: Default + PartialEq + Send + Sync + fmt::Debug + 'static;

  fn new(props: &Self::Props) -> Self;

  fn on_prop_change(&mut self, _props: &Self::Props) -> ShouldRebuild {
    ShouldRebuild::Yes
  }

  fn build(&mut self, _props: &Self::Props) -> Node {
    node::empty()
  }
}

pub trait UnitElement: Default + Send + Sync + fmt::Debug {
  fn build(&mut self) -> Node {
    node::empty()
  }
}

impl<T: UnitElement> Element for T {
  type Props = ();

  fn new(_: &Self::Props) -> Self {
    T::default()
  }

  fn build(&mut self, _: &Self::Props) -> Node {
    self.build()
  }
}

#[derive(Debug)]
pub enum ShouldRebuild {
  No,
  Yes,
}

#[derive(Default)]
pub struct RebuildRequired;

impl ecs::Component for RebuildRequired {
  type Storage = ecs::NullStorage<Self>;
}

pub fn create<E: Element + 'static>(res: &engine::Resources, props: E::Props) {
  let entities = res.fetch::<ecs::Entities>();

  let mut hierarchy = res.fetch_mut::<Hierarchy>();
  let mut mounts = ecs::write_components::<Mount>(res);
  let mut rebuild_required = ecs::write_components::<RebuildRequired>(res);

  hierarchy.roots.push(
    entities
      .build_entity()
      .with(Mount::new(InstanceBox::new::<E>(props)), &mut mounts)
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
          .iter()
          .map(|e| e.id())
          .collect::<Vec<_>>()
      );

      println!(
        "real_children: {:?}",
        mount
          .real_children
          .iter()
          .map(|e| e.id())
          .collect::<Vec<_>>()
      );
    }
  }
}
