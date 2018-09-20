// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use image::{load_from_memory, RgbaImage};

/// Image that can be drawn on the screen.
#[derive(Debug)]
pub struct Image {
  size: Vector2<f32>,
  rgba_image: RgbaImage,
  ggez_image: Mutex<Option<ggez::graphics::Image>>,
}

impl Image {
  /// Gets the size of the image.
  pub fn size(&self) -> Vector2<f32> {
    self.size
  }

  /// Draws the image on the screen.
  pub fn draw(&self, core: &mut Core, params: ggez::graphics::DrawParam) -> ggez::GameResult<()> {
    let mut ggez_image = self.ggez_image.lock().expect("could not lock Image::ggez");

    // Create the ggez image from the loaded image data if it has not yet been
    // created.
    if !ggez_image.is_some() {
      let mut image = ggez::graphics::Image::from_rgba8(
        &mut core.ctx,
        self.size.x as u16,
        self.size.y as u16,
        &self.rgba_image,
      )?;

      image.set_filter(ggez::graphics::FilterMode::Nearest);

      *ggez_image = Some(image);
    }

    ggez::graphics::draw(&mut core.ctx, ggez_image.as_ref().unwrap(), params)
  }
}

// Support loading images from files.
impl core::Asset for Image {
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
