// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::rendering;
use std::sync::Arc;

pub struct Image {
  texture: rendering::Texture,
}

impl Image {
  pub fn new(device: &Arc<rendering::Device>, bytes: &[u8]) -> Image {
    let rgba_image = image::load_from_memory(bytes)
      .expect("could not load image")
      .to_rgba();

    let (width, height) = rgba_image.dimensions();

    let texture = rendering::Texture::new(
      device,
      bytes,
      rendering::TextureFormat::Rgba8Srgb,
      width,
      height,
    );

    Image { texture }
  }
}
