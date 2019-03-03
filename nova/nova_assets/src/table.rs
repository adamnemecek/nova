// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{AssetId, AssetPath};
use nova_core::engine::Resources;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct AssetTable {
  entries: HashMap<AssetPath, AssetId>,
}

impl AssetTable {
  pub fn has(&self, path: &AssetPath) -> bool {
    self.entries.contains_key(path)
  }

  pub fn get(&self, path: &AssetPath) -> Option<AssetId> {
    self.entries.get(path).cloned()
  }

  pub(crate) fn insert(&mut self, path: AssetPath, id: AssetId) {
    self.entries.insert(path, id);
  }
}
