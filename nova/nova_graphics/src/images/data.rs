// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use image::{ImageFormat, RgbaImage};
use nova_core::components::{Component, HashMapStorage};
use nova_core::math::Size;
use nova_core::quick_error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ImageData {
  bytes: Arc<[u8]>,
  size: Size<u32>,
}

impl ImageData {
  pub fn load_file(path: impl AsRef<Path>) -> Result<Self, ImageLoadError> {
    let path = path.as_ref();

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

  pub fn load_bytes(bytes: &[u8]) -> Result<Self, ImageLoadError> {
    let data = image::load_from_memory(bytes)?;

    Ok(Self::from(data.to_rgba()))
  }

  pub fn bytes(&self) -> &Arc<[u8]> {
    &self.bytes
  }

  pub fn size(&self) -> Size<u32> {
    self.size
  }
}

impl Component for ImageData {
  type Storage = HashMapStorage<Self>;
}

impl PartialEq for ImageData {
  fn eq(&self, other: &ImageData) -> bool {
    Arc::ptr_eq(&self.bytes, &other.bytes)
  }
}

impl From<RgbaImage> for ImageData {
  fn from(rgba: RgbaImage) -> Self {
    Self {
      size: rgba.dimensions().into(),
      bytes: rgba.into_vec().into_boxed_slice().into(),
    }
  }
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
