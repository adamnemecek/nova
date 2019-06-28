// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::elements::ElementInstance;
use crate::nodes::ChildNodes;
use nova_core::components::{Component, HashMapStorage};
use nova_core::entities::Entity;

#[derive(Debug)]
pub struct Node {
  pub element: ElementInstance,
  pub parent: Option<Entity>,
  pub spec_children: ChildNodes,
  pub real_children: ChildNodes,
  pub should_rebuild: bool,
}

impl Node {
  pub fn new(element: ElementInstance, parent: Option<Entity>) -> Self {
    Node {
      element,
      parent,
      spec_children: ChildNodes::default(),
      real_children: ChildNodes::default(),
      should_rebuild: true,
    }
  }

  pub fn children<'a>(&'a self) -> impl Iterator<Item = Entity> + 'a {
    self.real_children.entities.iter().cloned()
  }
}

impl Component for Node {
  type Storage = HashMapStorage<Self>;
}
