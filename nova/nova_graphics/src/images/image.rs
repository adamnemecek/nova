// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::ImageError;
use ::image::RgbaImage;
use nova_core::ecs;
use nova_math::Size;

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

/*
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
        return Err(assets::LoadError::Other(Box::new(Error::UnsupportedError(
          format!("Image format image/{:?} is not supported.", format),
        ))));
      }
    };

    let file = fs.open(path)?;
    let reader = BufReader::new(file);

    let image = image::load(reader, format)
      .map_err(|err| match err {
        Error::IoError(err) => assets::LoadError::Io(err),
        _ => assets::LoadError::Other(Box::new(err)),
      })?
      .to_rgba();

    Ok(Self::from_rgba(image))
  }
}
*/
