use super::{Num, Scalar};
use derive_more::*;
use std::fmt;

/// Two-dimensional size with width and height.
#[derive(Clone, Copy, PartialEq, Eq, From)]
pub struct Size<T: Scalar> {
  pub width: T,
  pub height: T,
}

impl<T: Scalar> Size<T> {
  /// Creates a new size with the given width and height.
  pub fn new(width: T, height: T) -> Self {
    Size { width, height }
  }
}

impl From<Size<u32>> for Size<f32> {
  fn from(input: Size<u32>) -> Self {
    Size::new(input.width as f32, input.height as f32)
  }
}

impl From<gfx_hal::image::Extent> for Size<u32> {
  fn from(extent: gfx_hal::image::Extent) -> Self {
    Size::new(extent.width, extent.height)
  }
}

impl From<Size<u32>> for gfx_hal::image::Extent {
  fn from(size: Size<u32>) -> Self {
    gfx_hal::image::Extent {
      width: size.width,
      height: size.height,
      depth: 1,
    }
  }
}

impl From<gfx_hal::window::Extent2D> for Size<u32> {
  fn from(extent: gfx_hal::window::Extent2D) -> Self {
    Size::new(extent.width, extent.height)
  }
}

impl From<Size<u32>> for gfx_hal::window::Extent2D {
  fn from(size: Size<u32>) -> Self {
    gfx_hal::window::Extent2D {
      width: size.width,
      height: size.height,
    }
  }
}

impl<T: Scalar + Num> Default for Size<T> {
  fn default() -> Self {
    Size::new(T::zero(), T::zero())
  }
}

impl<T: Scalar + Num> fmt::Debug for Size<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({:?}, {:?})", &self.width, &self.height)
  }
}
