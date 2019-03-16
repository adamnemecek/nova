// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::elements::ElementPrototype;
use crate::nodes::{Node, NodeContext, NodeHierarchy};
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

  pub(crate) fn create_element(
    &mut self,
    prototype: ElementPrototype,
    ctx: NodeContext,
  ) -> &mut Node {
    let entity = ctx.entity;

    self
      .nodes
      .insert(entity, Node::new((prototype.new)(prototype.element, ctx)))
      .expect("Could not create element node");

    self
      .nodes
      .get_mut(entity)
      .expect("Could not get newly created element node")
  }

  pub(crate) fn delete(&mut self, entity: ecs::Entity) {
    self.nodes.remove(entity);
  }
}
