// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::error::Error;
use std::path::{Path, PathBuf};

use super::*;

/// Coordinates for a cell in an atlas.
pub type Cell = (usize, usize);

/// A single image divided into cells containing individual sprites.
#[derive(Debug)]
pub struct Atlas {
  /// Image to render cells from.
  pub texture: core::Texture,
  /// Width of a single cell in the atlas.
  pub cell_width: usize,
  /// Height of a single cell in the atlas.
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

impl core::Asset for Atlas {
  fn load(assets: &core::Assets, path: &Path) -> Result<Self, Box<dyn Error>> {
    let mut path = path.to_owned();

    let data = assets.load::<Data>(&path)?;

    path.pop();

    let texture = assets.load(&path.join(data.image))?;

    Ok(Atlas {
      texture,
      cell_width: data.cell_width,
      cell_height: data.cell_height,
    })
  }
}

/// Data for an atlas.
#[derive(Serialize, Deserialize)]
pub struct Data {
  /// Path to the image to use for the atlas.
  pub image: PathBuf,
  /// Width of a single cell in the atlas.
  pub cell_width: usize,
  /// Height of a single cell in the atlas.
  pub cell_height: usize,
}
