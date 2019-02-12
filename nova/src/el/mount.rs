// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::InstanceBox;
use crate::ecs;

#[derive(Debug)]
pub struct Mount {
  pub instance: InstanceBox,
  pub node_children: Vec<ecs::Entity>,
  pub real_children: Vec<ecs::Entity>,
}

impl Mount {
  pub fn new(instance: InstanceBox) -> Self {
    Mount {
      instance,
      node_children: Vec::new(),
      real_children: Vec::new(),
    }
  }
}

impl ecs::Component for Mount {
  type Storage = ecs::BTreeStorage<Self>;
}
