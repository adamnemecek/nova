// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova_core::entities::Entity;
use nova_core::resources::WriteResource;

pub type WriteNodeHierarchy<'a> = WriteResource<'a, NodeHierarchy>;

#[derive(Debug, Default)]
pub struct NodeHierarchy {
  pub roots: Vec<Entity>,
  pub sorted: Vec<Entity>,
}
