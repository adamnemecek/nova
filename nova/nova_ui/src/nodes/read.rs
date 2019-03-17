// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::nodes::{Node, NodeHierarchy};
use nova_core::ecs;
use nova_core::ecs::derive::*;

#[derive(SystemData)]
pub struct ReadNodes<'a> {
  pub(crate) hierarchy: ecs::ReadResource<'a, NodeHierarchy>,
  nodes: ecs::ReadComponents<'a, Node>,
}

impl<'a> ReadNodes<'a> {
  pub fn roots(&'a self) -> impl DoubleEndedIterator<Item = ecs::Entity> + 'a {
    self.hierarchy.roots.iter().cloned()
  }

  pub fn sorted(&'a self) -> impl DoubleEndedIterator<Item = ecs::Entity> + 'a {
    self.hierarchy.sorted.iter().cloned()
  }

  pub fn children_of(&'a self, entity: ecs::Entity) -> impl Iterator<Item = ecs::Entity> + 'a {
    match self.nodes.get(entity) {
      Some(node) => node.real_children.entities.iter().cloned(),
      None => [].iter().cloned(),
    }
  }
}
