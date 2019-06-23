// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
pub use ::image::ImageError as ImageDataLoadError;
use std::path::Path;

type BgraImage = ::image::ImageBuffer<::image::Bgra<u8>, Vec<u8>>;

pub struct ImageData(BgraImage);

impl ImageData {
  pub fn load_file(path: impl AsRef<Path>) -> Result<Self, ImageDataLoadError> {
    let image = ::image::open(path)?;

    Ok(Self(image.to_bgra()))
  }

  pub fn size(&self) -> Size<u32> {
    Size::new(self.0.width(), self.0.height())
  }
}

impl From<ImageData> for Vec<u8> {
  fn from(data: ImageData) -> Self {
    data.0.into_vec()
  }
}
