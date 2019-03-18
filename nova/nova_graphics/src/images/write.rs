// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Image, ImageId};
use nova_assets::{AssetId, AssetPath, ReadAssets};
use nova_core::ecs;
use nova_core::ecs::derive::*;

#[derive(SystemData)]
pub struct WriteImages<'a>(ecs::WriteComponents<'a, Image>);

impl<'a> WriteImages<'a> {
  pub fn get(&self, id: impl Into<ImageId>) -> Option<&Image> {
    self.0.get(id.into().into())
  }

  pub fn get_mut(&mut self, id: impl Into<ImageId>) -> Option<&mut Image> {
    self.0.get_mut(id.into().into())
  }

  pub fn insert(&mut self, image: Image) -> ImageId {
    let entity = self.0.fetched_entities().create();

    self.0.insert(entity, image).unwrap();

    ImageId(entity)
  }

  pub fn load_asset(&mut self, asset_id: AssetId, assets: &ReadAssets) -> ImageId {
    let asset = assets.get(asset_id);
    let entity = asset_id.into();

    if !self.0.contains(entity) {
      let image = Image::load(asset.fs_path()).expect("Could not load image");

      self.0.insert(entity, image).unwrap();
    }

    ImageId(entity)
  }

  pub fn load_asset_at_path(&mut self, path: &AssetPath, assets: &ReadAssets) -> ImageId {
    match assets.lookup(path) {
      Some(id) => self.load_asset(id, assets),
      None => panic!(format!("Asset path {:?} does not exist.", path)),
    }
  }
}
