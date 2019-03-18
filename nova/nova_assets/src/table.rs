// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{AssetId, AssetPath};
use nova_core::collections::FnvHashMap;

#[derive(Debug, Default)]
pub struct AssetTable {
  pub(crate) by_path: FnvHashMap<AssetPath, AssetId>,
}

impl AssetTable {
  pub fn contains(&self, path: &AssetPath) -> bool {
    self.by_path.contains_key(path)
  }

  pub fn get(&self, path: &AssetPath) -> Option<AssetId> {
    debug_assert!(path.has_root(), "The given asset path cannot be relative.");

    self.by_path.get(path).cloned()
  }
}
