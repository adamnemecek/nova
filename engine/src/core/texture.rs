// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use image;
use std::error::Error;
use std::ops::Deref;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Texture(Arc<Data>);

impl Deref for Texture {
  type Target = Data;

  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}

impl Asset for Texture {
  fn load(assets: &Assets, path: &Path) -> Result<Self, Box<dyn Error>> {
    let rgba_image = {
      use std::io::Read;

      let mut buf = Vec::new();
      let mut file = assets.open_file(path)?;

      file.read_to_end(&mut buf)?;
      image::load_from_memory(&buf)?.to_rgba()
    };

    let (width, height) = rgba_image.dimensions();

    let texture = Texture(Arc::new(Data {
      width: width as usize,
      height: height as usize,
      rgba_image,
      ggez_image: RwLock::new(None),
    }));

    assets.queue_resource_load(texture.clone());

    Ok(texture)
  }
}

#[derive(Debug)]
pub struct Data {
  pub width: usize,
  pub height: usize,
  pub rgba_image: image::RgbaImage,
  pub ggez_image: RwLock<Option<ggez::graphics::Image>>,
}
