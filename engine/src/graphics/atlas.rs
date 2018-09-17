// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use std::error::Error;
use std::path::{Path, PathBuf};

/// Coordinates for a cell in an atlas.
pub type Cell = (usize, usize);

/// Asset that divides a single texture into one or more cells.
#[derive(Debug)]
pub struct Atlas {
  /// Texture of the atlas.
  pub texture: core::Texture,
  /// Width of a single cell.
  pub cell_width: usize,
  /// Height of a single cell.
  pub cell_height: usize,
}

impl Atlas {
  /// Gets the source rectangle for a given `cell` in the atlas.
  pub fn get(&self, cell: Cell) -> ggez::graphics::Rect {
    let w = self.cell_width as f32 / self.texture.width as f32;
    let h = self.cell_height as f32 / self.texture.height as f32;

    let x = cell.0 as f32 * w;
    let y = cell.1 as f32 * h;

    ggez::graphics::Rect::new(x, y, w, h)
  }
}

// Support loading atlases from YAML files that reference image files.
impl core::Asset for Atlas {
  fn load(assets: &core::Assets, path: &Path) -> Result<Self, Box<dyn Error>> {
    let mut path = path.to_owned();

    // Load the atlas data.
    let data = assets.load::<Data>(&path)?;

    path.pop();

    // Load the texture referenced in the data.
    let texture = assets.load(&path.join(data.texture))?;

    Ok(Atlas {
      texture,
      cell_width: data.cell_width,
      cell_height: data.cell_height,
    })
  }
}

/// Serializable data for an `Atlas` asset.
#[derive(Serialize, Deserialize)]
pub struct Data {
  /// Relative path to the atlas texture.
  pub texture: PathBuf,
  /// Width of a single cell in the atlas.
  pub cell_width: usize,
  /// Height of a single cell in the atlas.
  pub cell_height: usize,
}
