// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova_core::ecs;
use std::collections::BTreeSet;

#[derive(Debug, Default)]
pub struct ChildNodes {
  pub entities: Vec<ecs::Entity>,
  pub references: BTreeSet<ecs::Entity>,
}
