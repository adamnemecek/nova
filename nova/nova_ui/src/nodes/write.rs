// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::elements::ElementInstance;
use crate::nodes::hierarchy::WriteNodeHierarchy;
use crate::nodes::Node;
use nova_core::components::WriteComponents;
use nova_core::entities::Entity;
use nova_core::systems::derive::*;
use std::iter::DoubleEndedIterator;

#[derive(SystemData)]
pub struct WriteNodes<'a> {
  pub(crate) hierarchy: WriteNodeHierarchy<'a>,
  nodes: WriteComponents<'a, Node>,
}

impl<'a> WriteNodes<'a> {
  pub fn roots(&'a self) -> impl DoubleEndedIterator<Item = Entity> + 'a {
    self.hierarchy.roots.iter().cloned()
  }

  pub fn get_mut(&mut self, entity: Entity) -> Option<&mut Node> {
    self.nodes.get_mut(entity)
  }

  pub(crate) fn create_on_entity(
    &mut self,
    entity: Entity,
    element: ElementInstance,
    parent: Option<Entity>,
  ) -> &mut Node {
    self
      .nodes
      .insert(entity, Node::new(element, parent))
      .unwrap();

    self.nodes.get_mut(entity).unwrap()
  }

  pub(crate) fn delete(&mut self, entity: Entity) {
    self.nodes.remove(entity);
  }
}
