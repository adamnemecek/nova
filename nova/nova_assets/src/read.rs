// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Asset, AssetId, AssetPath, AssetTable};
use nova_core::ecs;
use nova_core::ecs::derive::*;

#[derive(SystemData)]
pub struct ReadAssets<'a> {
  assets: ecs::ReadComponents<'a, Asset>,
  table: ecs::ReadResource<'a, AssetTable>,
}

impl<'a> ReadAssets<'a> {
  pub fn get(&self, id: AssetId) -> Option<&Asset> {
    self.assets.get(id.0)
  }

  pub fn lookup_id(&self, path: &AssetPath) -> Option<AssetId> {
    self.table.get(path)
  }
}
