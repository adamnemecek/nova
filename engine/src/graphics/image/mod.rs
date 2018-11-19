// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod loader;
mod sampler;
mod source;

pub use self::loader::Loader;
pub use self::sampler::Sampler;
pub use self::source::Source;
pub use gfx_hal::format::Format;
pub use gfx_hal::image::Layout;

use super::device::{self, Device};
use crate::graphics::prelude::*;
use crate::math::Size;
use crate::utils::Droppable;
use gfx_memory::Factory;
use std::sync::Arc;

type AllocatorImage = <device::Allocator as Factory<Backend>>::Image;

pub struct Image {
  device: Arc<Device>,
  backing: Droppable<Backing>,
  view: Droppable<backend::ImageView>,
  size: Size<u32>,
}

impl Image {
  pub fn from_raw(device: &Arc<Device>, backing: Backing, format: Format, size: Size<u32>) -> Self {
    let view = device
      .raw()
      .create_image_view(
        backing.as_ref(),
        hal::image::ViewKind::D2,
        format,
        hal::format::Swizzle::NO,
        hal::image::SubresourceRange {
          aspects: hal::format::Aspects::COLOR,
          levels: 0..1,
          layers: 0..1,
        },
      )
      .expect("could not create image view");

    Image {
      device: device.clone(),
      backing: backing.into(),
      view: view.into(),
      size,
    }
  }

  pub fn size(&self) -> Size<u32> {
    self.size
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
