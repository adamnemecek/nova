// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova_core::collections::HashSet;
use nova_core::entities::Entity;

#[derive(Debug, Default)]
pub struct ChildNodes {
  pub entities: Vec<Entity>,
  pub references: HashSet<Entity>,
}
