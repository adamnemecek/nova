// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Asset, AssetId};
use nova_core::ecs;
use nova_core::ecs::derive::*;
use std::ops::Deref;

#[derive(SystemData)]
pub struct ReadAssets<'a>(ecs::ReadComponents<'a, Asset>);

impl<'a> ReadAssets<'a> {
  pub fn get(&self, id: AssetId) -> Option<&Asset> {
    self.0.get(id.0)
  }
}

impl<'a> Deref for ReadAssets<'a> {
  type Target = ecs::ReadComponents<'a, Asset>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
