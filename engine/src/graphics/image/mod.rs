mod data;
mod loader;
mod sampler;

pub use self::data::*;
pub use self::loader::Loader;
pub use self::sampler::Sampler;
pub use gfx_hal::format::Format;
pub use gfx_hal::image::Layout;

use super::device::{self, Device};
use super::hal::*;
use crate::utils::Droppable;
use gfx_memory::Factory;
use std::sync::Arc;

pub type Allocation = <device::Allocator as Factory<Backend>>::Image;

pub struct Image {
  inner: Droppable<Inner>,
  device: Arc<Device>,
}

struct Inner {
  view: backend::ImageView,
  image: Allocation,
}

impl AsRef<backend::ImageView> for Image {
  fn as_ref(&self) -> &backend::ImageView {
    &self.inner.view
  }
}

impl Drop for Image {
  fn drop(&mut self) {
    let device = &self.device;
    let raw_device = device.raw();

    if let Some(inner) = self.inner.take() {
      raw_device.destroy_image_view(inner.view);
      device.allocator().destroy_image(raw_device, inner.image);
    }
  }
}
