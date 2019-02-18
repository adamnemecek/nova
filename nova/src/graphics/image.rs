// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::math::Size;
use image::RgbaImage;
use std::path::Path;

pub use image::ImageError;

pub struct Image {
  bytes: Vec<u8>,
  size: Size<u32>,
}

impl Image {
  pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ImageError> {
    let image = image::open(path)?.to_rgba();

    Ok(Self::from_rgba(image))
  }

  fn from_rgba(image: RgbaImage) -> Self {
    Image {
      size: image.dimensions().into(),
      bytes: image.into_vec(),
    }
  }

  pub fn bytes(&self) -> &[u8] {
    &self.bytes
  }

  pub fn size(&self) -> Size<u32> {
    self.size
  }
}
