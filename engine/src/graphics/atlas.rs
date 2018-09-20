// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Coordinates for a cell in an atlas.
pub type AtlasCell = (usize, usize);

/// Asset that divides a single texture into one or more cells.
#[derive(Debug)]
pub struct Atlas {
  /// Image of the atlas.
  pub image: Image,
  /// Width of a single cell.
  pub cell_width: usize,
  /// Height of a single cell.
  pub cell_height: usize,
  /// Center of a cell where `(0.0, 0.0)` is the top left corner and
  /// `(cell_width, cell_height)` is the bottom right corner.
  pub cell_origin: Vector2<f32>,
}

impl Atlas {
  /// Gets the source rectangle for a given `cell` in the atlas.
  pub fn get(&self, cell: AtlasCell) -> ggez::graphics::Rect {
    let size = self.image.size();

    let w = self.cell_width as f32 / size.x;
    let h = self.cell_height as f32 / size.y;

    let x = cell.0 as f32 * w;
    let y = cell.1 as f32 * h;

    ggez::graphics::Rect::new(x, y, w, h)
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
      cell_width: data.cell_width,
      cell_height: data.cell_height,
      cell_origin: Vector2::new(data.cell_origin.0, data.cell_origin.1),
    })
  }
}

/// Serializable data for an `Atlas` asset.
#[derive(Serialize, Deserialize)]
pub struct AtlasData {
  /// Relative path to the atlas image.
  pub image: assets::PathBuf,
  /// Width of a single cell in the atlas.
  pub cell_width: usize,
  /// Height of a single cell in the atlas.
  pub cell_height: usize,
  /// Center of a cell where `(0.0, 0.0)` is the top left corner and
  /// `(cell_width, cell_height)` is the bottom right corner.
  pub cell_origin: (f32, f32),
}
