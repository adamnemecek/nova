mod loader;
mod sampler;
mod source;

pub use self::loader::Loader;
pub use self::sampler::Sampler;
pub use self::source::Source;
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
  backing: Droppable<Backing>,
  device: Arc<Device>,
}

impl Image {
  pub unsafe fn new(device: &Arc<Device>, backing: Backing, format: Format) -> Self {
    let view = device
      .raw()
      .create_image_view(
        backing.as_ref(),
        gfx_hal::image::ViewKind::D2,
        format,
        gfx_hal::format::Swizzle::NO,
        gfx_hal::image::SubresourceRange {
          aspects: gfx_hal::format::Aspects::COLOR,
          levels: 0..1,
          layers: 0..1,
        },
      )
      .expect("could not create image view");

    Image {
      view: view.into(),
      backing: backing.into(),
      device: device.clone(),
    }
  }
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

    if let Some(backing) = self.backing.take() {
      backing.destroy(device);
    }
  }
}

pub enum Backing {
  Swapchain(backend::Image),
  Allocated(AllocatorImage),
}

impl Backing {
  fn destroy(self, device: &Device) {
    match self {
      Backing::Swapchain(_) => {}
      Backing::Allocated(image) => device.allocator().destroy_image(device.raw(), image),
    }
  }
}

impl AsRef<backend::Image> for Backing {
  fn as_ref(&self) -> &backend::Image {
    match self {
      Backing::Swapchain(image) => image,
      Backing::Allocated(image) => image.raw(),
    }
  }
}
