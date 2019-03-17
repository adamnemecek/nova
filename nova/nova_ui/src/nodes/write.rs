// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::elements::ElementInstance;
use crate::nodes::{Node, NodeHierarchy};
use nova_core::ecs;
use nova_core::ecs::derive::*;
use std::iter::DoubleEndedIterator;

#[derive(SystemData)]
pub struct WriteNodes<'a> {
  pub(crate) hierarchy: ecs::WriteResource<'a, NodeHierarchy>,
  nodes: ecs::WriteComponents<'a, Node>,
}

impl<'a> WriteNodes<'a> {
  pub fn roots(&'a self) -> impl DoubleEndedIterator<Item = ecs::Entity> + 'a {
    self.hierarchy.roots.iter().cloned()
  }

  pub fn get_mut(&mut self, entity: ecs::Entity) -> Option<&mut Node> {
    self.nodes.get_mut(entity)
  }

  pub(crate) fn create_on_entity(
    &mut self,
    entity: ecs::Entity,
    element: ElementInstance,
    parent: Option<ecs::Entity>,
  ) -> &mut Node {
    self
      .nodes
      .insert(entity, Node::new(element, parent))
      .unwrap();

    self.nodes.get_mut(entity).unwrap()
  }

  pub(crate) fn delete(&mut self, entity: ecs::Entity) {
    self.nodes.remove(entity);
  }
}
