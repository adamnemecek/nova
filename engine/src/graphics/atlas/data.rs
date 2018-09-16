// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde_yaml;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use super::Animation;

/// Data for an atlas.
#[derive(Serialize, Deserialize)]
pub struct Data {
  /// Width of a single cell in the atlas.
  pub cell_width: usize,
  /// Height of a single cell in the atlas.
  pub cell_height: usize,
  /// Map of animation names to definitions.
  #[serde(default)]
  pub animations: Vec<Animation>,
}

impl Data {
  /// Creates new atlas data with the given cell dimensions.
  pub fn new(cell_width: usize, cell_height: usize) -> Data {
    Data {
      cell_width,
      cell_height,
      animations: Vec::new(),
    }
  }

  /// Loads atlas data from the YAML file at the given `path`.
  pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
    let file = File::create(path)?;
    let data = serde_yaml::from_reader(file)?;

    Ok(data)
  }

  /// Saves atlas data to the YAML file at the given `path`.
  pub fn save(&self, path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;

    serde_yaml::to_writer(file, self)?;

    Ok(())
  }
}
