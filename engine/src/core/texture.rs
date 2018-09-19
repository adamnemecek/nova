// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use image;
use std::error::Error;
use std::ops::Deref;
use std::path::Path;

/// Asset loaded from images for use in engine graphics.
#[derive(Debug, Clone)]
pub struct Texture(Arc<Data>);

// Make `TextureData` directly accessible from a `Texture` so the `Arc` is
// transparent to the user.
impl Deref for Texture {
  type Target = Data;

  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}

// Support loading textures from image assets.
impl Asset for Texture {
  fn load(assets: &Assets, path: &Path) -> Result<Self, Box<dyn Error>> {
    // Load an RGBA8 image from the file using the `image` crate.
    let rgba_image = {
      use std::io::Read;

      let mut buf = Vec::new();
      let mut file = assets.open_file(path)?;

      file.read_to_end(&mut buf)?;
      image::load_from_memory(&buf)?.to_rgba()
    };

    // Get image dimensions.
    let (width, height) = rgba_image.dimensions();

    // Create a texture asset with no ggez image.
    let texture = Texture(Arc::new(Data {
      width: width as f32,
      height: height as f32,
      rgba_image,
      ggez_image: RwLock::new(None),
    }));

    // Queue the ggez image to be loaded next tick using the `Assets` resource.
    assets.queue_resource_load(texture.clone());

    Ok(texture)
  }
}

/// Actual data of a `Texture`.
#[derive(Debug)]
pub struct Data {
  /// Width of the texture in pixels.
  pub width: f32,
  /// Height of the texture in pixels.
  pub height: f32,
  /// Raw RGBA8 image data of the texture.
  pub rgba_image: image::RgbaImage,
  /// Low-level image loaded to the device by ggez.
  ///
  /// This starts empty until the image is loaded by the `Assets` resource.
  pub ggez_image: RwLock<Option<ggez::graphics::Image>>,
}
