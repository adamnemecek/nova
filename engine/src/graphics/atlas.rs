// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Image;
use crate::assets;
use crate::prelude::*;

/// Asset that divides a single texture into one or more cells.
#[derive(Debug)]
pub struct Atlas {
  /// Image of the atlas.
  pub image: Image,
  /// Size of a single cell.
  pub cell_size: Vector2<u16>,
  /// Center of a cell where `(0.0, 0.0)` is the top left corner and
  /// `(cell_width, cell_height)` is the bottom right corner.
  pub cell_origin: Vector2<f32>,
}

impl Atlas {
  /// Gets the source rectangle for a given `cell` in the atlas.
  pub fn get(&self, cell: Vector2<u16>) -> Rect<f32> {
    let size = self.image.size();

    let w = self.cell_size.x as f32 / size.x as f32;
    let h = self.cell_size.y as f32 / size.y as f32;

    let x = cell.x as f32 * w;
    let y = cell.y as f32 * h;

    Rect::new(x, y, w, h)
  }
}

// Support loading atlases from YAML files that reference image files.
impl assets::Asset for Atlas {
  fn load(fs: &assets::OverlayFs, path: &assets::Path) -> Result<Self, assets::Error> {
    let mut path = path.to_owned();

    // Load the atlas data.
    let data = fs.load::<AtlasData>(&path)?;

    path.pop();

    // Load the image referenced in the data.
    let image = fs.load(&path.join(data.image))?;

    Ok(Atlas {
      image,
      cell_size: Vector2::new(data.cell_size.0, data.cell_size.1),
      cell_origin: Vector2::new(data.cell_origin.0, data.cell_origin.1),
    })
  }
}

/// Serializable data for an `Atlas` asset.
#[derive(Serialize, Deserialize)]
pub struct AtlasData {
  /// Relative path to the atlas image.
  pub image: assets::PathBuf,
  /// Size of a single cell in the atlas.
  pub cell_size: (u16, u16),
  /// Center of a cell where `(0.0, 0.0)` is the top left corner and
  /// `(cell_width, cell_height)` is the bottom right corner.
  pub cell_origin: (f32, f32),
}
