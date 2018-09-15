// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ggez::graphics::{FilterMode, Image, Rect};
use serde_yaml;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

use prelude::*;

/// An image split into one or more cells.
///
/// Also known as a spritesheet.
pub struct Atlas {
  /// Image data for the atlas.
  pub image: Image,
  /// List of cells in the atlas, where each cell is a rectangular slice of
  /// the entire image.
  pub cells: Vec<Rect>,
}

impl Atlas {
  pub fn load(game: &mut Game, path: impl Into<PathBuf>) -> Result<Self, Box<dyn Error>> {
    let mut path = path.into();

    // Append `.png` to the path and load it as the image.
    path.set_extension("png");

    let mut image = Image::new(&mut game.platform.ctx, &path)?;

    image.set_filter(FilterMode::Nearest);

    // Append `.yml` to the path and attempt to load it.
    path.set_extension("yml");

    if let Ok(file) = ggez::filesystem::open(&mut game.platform.ctx, &path) {
      // Deserialize the file as `Data`.
      let data = serde_yaml::from_reader::<_, Data>(file)?;

      // Convert the data into an atlas with the loaded image.
      Ok(data.into_atlas(image))
    } else {
      // Return an atlas with one frame covering the entire loaded image.
      Ok(Atlas {
        image,
        cells: vec![Rect::new(0.0, 0.0, 1.0, 1.0)],
      })
    }
  }
}

/// Data loaded from a YAML file associated with an atlas.
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
  /// Number of columns in the grid that defines the frames of the atlas.
  pub columns: u16,
  /// Number of rows in the grid that defines the frames of the atlas.
  pub rows: u16,
}

impl Data {
  pub fn save(&self, path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;

    serde_yaml::to_writer(file, self)?;

    Ok(())
  }

  /// Converts atlas data into an atlas by calculating the rects of each frame.
  fn into_atlas(self, image: Image) -> Atlas {
    let mut cells = Vec::with_capacity((self.columns * self.rows) as usize);

    let w = 1.0 / self.columns as f32;
    let h = 1.0 / self.rows as f32;

    for y in 0..self.rows {
      for x in 0..self.columns {
        cells.push(Rect::new(x as f32 * w, y as f32 * h, w, h));
      }
    }

    Atlas { image, cells }
  }
}
