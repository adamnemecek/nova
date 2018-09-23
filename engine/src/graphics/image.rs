// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::assets;
use crate::prelude::*;
use std::sync::Mutex;

/// Image that can be drawn on the screen.
#[derive(Debug)]
pub struct Image {
  /// Size of the image in pixels.
  size: Vector2<u16>,
  /// Image data loaded to memory.
  pub(crate) rgba_image: ::image::RgbaImage,
  /// Underlying ggez image if it is loaded.
  pub(crate) ggez_image: Mutex<Option<ggez::graphics::Image>>,
}

impl Image {
  pub fn new(bytes: &[u8]) -> Result<Self, assets::Error> {
    let rgba_image = ::image::load_from_memory(bytes)?.to_rgba();
    let (width, height) = rgba_image.dimensions();

    Ok(Image {
      size: Vector2::new(width as u16, height as u16),
      rgba_image: rgba_image,
      ggez_image: Mutex::new(None),
    })
  }

  /// Gets the size of the image in pixels.
  pub fn size(&self) -> Vector2<u16> {
    self.size
  }
}

// Support loading images from files.
impl assets::Asset for Image {
  fn load(fs: &assets::OverlayFs, path: &assets::Path) -> Result<Self, assets::Error> {
    use std::io::Read;

    let mut buffer = Vec::new();

    {
      let mut file = fs.open(path)?;

      file.read_to_end(&mut buffer)?;
    }

    Image::new(&buffer)
  }
}
