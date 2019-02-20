// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::assets;
use crate::math::Size;
use image::RgbaImage;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Arc;

pub use image::{ImageError, ImageFormat};

#[derive(Debug, Clone)]
pub struct Image(Arc<Inner>);

#[derive(Debug)]
struct Inner {
  bytes: Vec<u8>,
  size: Size<u32>,
}

impl Image {
  fn from_rgba(image: RgbaImage) -> Self {
    Image(Arc::new(Inner {
      size: image.dimensions().into(),
      bytes: image.into_vec(),
    }))
  }

  pub fn bytes(&self) -> &[u8] {
    &self.0.bytes
  }

  pub fn size(&self) -> Size<u32> {
    self.0.size
  }
}

impl assets::Load for Image {
  fn load(path: PathBuf, fs: &assets::OverlayFs) -> assets::LoadResult<Self> {
    let ext = path
      .extension()
      .and_then(|s| s.to_str())
      .map_or("".to_string(), |s| s.to_ascii_lowercase());

    let format = match &ext[..] {
      "jpg" | "jpeg" => ImageFormat::JPEG,
      "png" => ImageFormat::PNG,
      "gif" => ImageFormat::GIF,
      "webp" => ImageFormat::WEBP,
      "tif" | "tiff" => ImageFormat::TIFF,
      "tga" => ImageFormat::TGA,
      "bmp" => ImageFormat::BMP,
      "ico" => ImageFormat::ICO,
      "hdr" => ImageFormat::HDR,
      "pbm" | "pam" | "ppm" | "pgm" => ImageFormat::PNM,
      format => {
        return Err(assets::LoadError::Other(Box::new(
          ImageError::UnsupportedError(format!(
            "Image format image/{:?} is not supported.",
            format
          )),
        )));
      }
    };

    let file = fs.open(path)?;
    let reader = BufReader::new(file);

    let image = image::load(reader, format)
      .map_err(|err| match err {
        ImageError::IoError(err) => assets::LoadError::Io(err),
        _ => assets::LoadError::Other(Box::new(err)),
      })?
      .to_rgba();

    Ok(Self::from_rgba(image))
  }
}

impl PartialEq for Image {
  fn eq(&self, other: &Self) -> bool {
    Arc::ptr_eq(&self.0, &other.0)
  }
}
