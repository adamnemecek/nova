// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Hierarchy, Node};
use crate::ecs;
use crate::ecs::derive::*;
use std::ops::Deref;

#[derive(SystemData)]
pub struct ReadHierarchyNodes<'a> {
  hierarchy: ecs::ReadResource<'a, Hierarchy>,
  pub nodes: ecs::ReadComponents<'a, Node>,
}

impl<'a> ReadHierarchyNodes<'a> {
  pub fn get_children_of<'b>(
    &'b self,
    entity: ecs::Entity,
  ) -> impl Iterator<Item = ecs::Entity> + 'b {
    let node = self.nodes.get(entity);

    node.map(Node::children).into_iter().flatten()
  }
}

impl<'a> Deref for ReadHierarchyNodes<'a> {
  type Target = Hierarchy;

  fn deref(&self) -> &Hierarchy {
    &self.hierarchy
  }
}
