// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::InstanceBox;
use crate::ecs;
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Mount {
  pub(super) instance: InstanceBox,
  pub node_children: Children,
  pub real_children: Children,
}

impl Mount {
  pub(super) fn new(instance: InstanceBox) -> Self {
    Mount {
      instance,
      node_children: Children::default(),
      real_children: Children::default(),
    }
  }
}

impl ecs::Component for Mount {
  type Storage = ecs::BTreeStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Children {
  pub entities: Vec<ecs::Entity>,
  pub references: BTreeSet<ecs::Entity>,
}
