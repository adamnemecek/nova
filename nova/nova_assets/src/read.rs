// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Asset, AssetId, AssetPath, AssetTable};
use nova_core::ecs;
use nova_core::ecs::derive::*;
use std::path::Path;

#[derive(SystemData)]
pub struct ReadAssets<'a> {
  assets: ecs::ReadComponents<'a, Asset>,
  table: ecs::ReadResource<'a, AssetTable>,
}

impl<'a> ReadAssets<'a> {
  pub fn get(&self, id: AssetId) -> &Asset {
    self.assets.get(id.0).expect("Asset ID does not exist.")
  }

  pub fn lookup(&self, path: &AssetPath) -> Option<AssetId> {
    self.table.get(path)
  }

  pub fn fs_path_of(&self, id: AssetId) -> &Path {
    self.get(id).fs_path()
  }
}
