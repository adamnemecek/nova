// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ObjectTemplate {
  pub atlas: Arc<graphics::Atlas>,
  pub cardinal_dirs_only: bool,
  pub animations: Vec<Animation>,
}

impl core::Asset for ObjectTemplate {
  fn load(assets: &core::Assets, path: &Path) -> Result<Self, Box<dyn Error>> {
    let mut path = path.to_owned();

    let data = assets.load::<Data>(&path)?;

    path.pop();

    let atlas = assets.load(&path.join(&data.atlas))?;

    Ok(ObjectTemplate {
      atlas: Arc::new(atlas),
      cardinal_dirs_only: data.cardinal_dirs_only,
      animations: data.animations.into_iter().map(Animation::from).collect(),
    })
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
  pub atlas: PathBuf,
  pub animations: Vec<animation::Data>,
  #[serde(default)]
  pub cardinal_dirs_only: bool,
}
