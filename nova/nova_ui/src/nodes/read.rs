// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::nodes::{Node, NodeHierarchy};
use nova_core::components::ReadComponents;
use nova_core::entities::Entity;
use nova_core::resources::ReadResource;
use nova_core::systems::derive::*;

#[derive(SystemData)]
pub struct ReadNodes<'a> {
  pub(crate) hierarchy: ReadResource<'a, NodeHierarchy>,
  nodes: ReadComponents<'a, Node>,
}

impl<'a> ReadNodes<'a> {
  pub fn roots(&'a self) -> impl DoubleEndedIterator<Item = Entity> + 'a {
    self.hierarchy.roots.iter().cloned()
  }

  pub fn sorted(&'a self) -> impl DoubleEndedIterator<Item = Entity> + 'a {
    self.hierarchy.sorted.iter().cloned()
  }

  pub fn children_of(&'a self, entity: Entity) -> impl Iterator<Item = Entity> + 'a {
    match self.nodes.get(entity) {
      Some(node) => node.real_children.entities.iter().cloned(),
      None => [].iter().cloned(),
    }
  }
}
