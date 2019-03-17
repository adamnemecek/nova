// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::elements::ElementInstance;
use crate::nodes::ChildNodes;
use nova_core::ecs;

#[derive(Debug)]
pub struct Node {
  pub(crate) element: ElementInstance,
  pub(crate) parent: Option<ecs::Entity>,
  pub(crate) spec_children: ChildNodes,
  pub(crate) real_children: ChildNodes,
  pub(crate) should_rebuild: bool,
}

impl Node {
  pub(crate) fn new(element: ElementInstance, parent: Option<ecs::Entity>) -> Self {
    Node {
      element,
      parent,
      spec_children: ChildNodes::default(),
      real_children: ChildNodes::default(),
      should_rebuild: true,
    }
  }

  pub fn children<'a>(&'a self) -> impl Iterator<Item = ecs::Entity> + 'a {
    self.real_children.entities.iter().cloned()
  }
}

impl ecs::Component for Node {
  type Storage = ecs::HashMapStorage<Self>;
}
