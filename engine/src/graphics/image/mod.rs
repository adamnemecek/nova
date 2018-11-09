mod loader;
mod sampler;
mod source;

pub use self::loader::Loader;
pub use self::sampler::Sampler;
pub use self::source::*;
pub use gfx_hal::format::Format;
pub use gfx_hal::image::Layout;

use super::device::{self, Device};
use super::hal::*;
use crate::utils::Droppable;
use gfx_memory::Factory;
use std::sync::Arc;

type AllocatorImage = <device::Allocator as Factory<Backend>>::Image;

pub struct Image {
  view: Droppable<backend::ImageView>,
  inner: Droppable<AllocatorImage>,
  device: Arc<Device>,
}

impl AsRef<backend::ImageView> for Image {
  fn as_ref(&self) -> &backend::ImageView {
    &self.view
  }
}

impl Drop for Image {
  fn drop(&mut self) {
    let device = &self.device;

    if let Some(view) = self.view.take() {
      device.raw().destroy_image_view(view);
    }

    if let Some(inner) = self.inner.take() {
      device.allocator().destroy_image(device.raw(), inner);
    }
  }
}
