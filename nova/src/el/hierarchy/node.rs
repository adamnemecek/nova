// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Instance;
use crate::ecs;
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Node {
  pub(crate) instance: Instance,
  pub(crate) spec_children: Children,
  pub(crate) real_children: Children,
  pub(crate) needs_build: bool,
}

impl Node {
  pub(crate) fn new(instance: Instance) -> Self {
    Node {
      instance,
      spec_children: Children::default(),
      real_children: Children::default(),
      needs_build: true,
    }
  }

  pub fn children<'a>(&'a self) -> impl Iterator<Item = ecs::Entity> + 'a {
    self.real_children.entities.iter().cloned()
  }
}

impl ecs::Component for Node {
  type Storage = ecs::HashMapStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Children {
  pub entities: Vec<ecs::Entity>,
  pub references: BTreeSet<ecs::Entity>,
}
