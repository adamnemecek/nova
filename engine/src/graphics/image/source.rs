use crate::math::algebra::Vector2;
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

  pub fn size(&self) -> Vector2<u32> {
    let dim = self.0.dimensions();

    Vector2::new(dim.0, dim.1)
  }
}