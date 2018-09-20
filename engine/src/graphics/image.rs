// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use image::{load_from_memory, RgbaImage};

/// Image that can be drawn on the screen.
#[derive(Debug)]
pub struct Image {
  size: Vector2<f32>,
  /// Image data loaded to memory.
  pub(crate) rgba_image: RgbaImage,
  /// Underlying ggez image if it is loaded.
  pub(crate) ggez_image: Mutex<Option<ggez::graphics::Image>>,
}

impl Image {
  /// Gets the size of the image in pixels.
  pub fn size(&self) -> Vector2<f32> {
    self.size
  }
}

// Support loading images from files.
impl assets::Asset for Image {
  fn load(fs: &assets::OverlayFs, path: &assets::Path) -> Result<Self, assets::Error> {
    let rgba_image = {
      use std::io::Read;

      let mut buf = Vec::new();
      let mut file = fs.open(path)?;

      file.read_to_end(&mut buf)?;

      load_from_memory(&buf)?.to_rgba()
    };

    let (width, height) = rgba_image.dimensions();

    Ok(Image {
      size: Vector2::new(width as f32, height as f32),
      rgba_image: rgba_image,
      ggez_image: Mutex::new(None),
    })
  }
}
