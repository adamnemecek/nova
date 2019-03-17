// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{ImageError, ImageFormat};
use ::image::RgbaImage;
use nova_core::ecs;
use nova_core::math::Size;
use nova_core::quick_error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Image {
  bytes: Box<[u8]>,
  size: Size<u32>,
}

impl From<RgbaImage> for Image {
  fn from(rgba: RgbaImage) -> Self {
    Image {
      size: rgba.dimensions().into(),
      bytes: rgba.into_vec().into_boxed_slice(),
    }
  }
}

impl Image {
  pub fn load(path: &Path) -> Result<Self, ImageLoadError> {
    let ext = path
      .extension()
      .and_then(OsStr::to_str)
      .unwrap_or("")
      .to_ascii_lowercase();

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

      _ => {
        return Err(ImageLoadError::UnknownExtension(ext));
      }
    };

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let image = image::load(reader, format)?;

    Ok(Self::from(image.to_rgba()))
  }

  pub fn from_bytes(bytes: &[u8]) -> Result<Self, ImageError> {
    let data = image::load_from_memory(bytes)?;

    Ok(Self::from(data.to_rgba()))
  }

  pub fn bytes(&self) -> &[u8] {
    &self.bytes
  }

  pub fn size(&self) -> Size<u32> {
    self.size
  }
}

impl ecs::Component for Image {
  type Storage = ecs::HashMapStorage<Self>;
}

quick_error! {
  #[derive(Debug)]
  pub enum ImageLoadError {
    Io(err: io::Error) {
      from()
      display("io error: {}", err)
    }
    UnknownExtension(ext: String) {
      from()
      display("unknown file extension {:?}", ext)
    }
    Image(err: ::image::ImageError) {
      from()
      display("{}", err)
    }
  }
}
