// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::math::Size;
use derive_more::*;
use image::RgbaImage;

pub type SourceError = ::image::ImageError;

#[derive(From)]
pub struct Source(RgbaImage);

impl Source {
  pub fn from_bytes(bytes: &[u8]) -> Result<Source, SourceError> {
    Ok(::image::load_from_memory(bytes)?.to_rgba().into())
  }

  pub fn bytes(&self) -> &[u8] {
    &self.0
  }

  pub fn size(&self) -> Size<u32> {
    self.0.dimensions().into()
  }
}
