// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod cache;

pub use self::cache::{TextureCache, TextureId};
pub use gfx_hal::format::Format as TextureFormat;
pub use gfx_hal::image::Access as TextureAccess;
pub use gfx_hal::image::Layout as TextureLayout;

use super::alloc::{Allocator, Memory, MemoryKind};
use super::device::{Device, DeviceExt};
use super::pipeline::MemoryBarrier;
use super::Backend;
use nova_math::Size;

pub type RawTexture = <Backend as gfx_hal::Backend>::Image;
pub type RawTextureView = <Backend as gfx_hal::Backend>::ImageView;
pub type Sampler = <Backend as gfx_hal::Backend>::Sampler;

#[derive(Debug)]
pub struct Texture {
  pub(crate) raw: RawTexture,
  pub(crate) raw_view: RawTextureView,
  memory: Memory,
  size: Size<u32>,
}

impl Texture {
  pub(crate) fn new(device: &Device, allocator: &mut Allocator, size: Size<u32>) -> Self {
    const FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;

    let mut raw = unsafe {
      device
        .create_image(
          gfx_hal::image::Kind::D2(size.width, size.height, 1, 1),
          1,
          FORMAT,
          gfx_hal::image::Tiling::Optimal,
          gfx_hal::image::Usage::TRANSFER_DST | gfx_hal::image::Usage::SAMPLED,
          gfx_hal::image::ViewCapabilities::empty(),
        )
        .expect("Could not create image")
    };

    let memory = allocator.alloc(device, MemoryKind::Gpu, unsafe {
      device.get_image_requirements(&raw)
    });

    unsafe {
      device
        .bind_image_memory(&memory, 0, &mut raw)
        .expect("Could not bind texture memory");
    }

    let raw_view = create_view(device, &raw, FORMAT);

    Texture {
      raw,
      raw_view,
      memory,
      size,
    }
  }

  pub fn size(&self) -> Size<u32> {
    self.size
  }

  pub(crate) fn barrier(
    &self,
    access_change: (TextureAccess, TextureAccess),
    layout_change: (TextureLayout, TextureLayout),
  ) -> MemoryBarrier<Backend> {
    MemoryBarrier::Image {
      families: None,
      target: &self.raw,
      states: (access_change.0, layout_change.0)..(access_change.1, layout_change.1),
      range: gfx_hal::image::SubresourceRange {
        aspects: gfx_hal::format::Aspects::COLOR,
        levels: 0..1,
        layers: 0..1,
      },
    }
  }

  pub fn destroy(self, device: &Device, allocator: &mut Allocator) {
    unsafe {
      device.destroy_image_view(self.raw_view);
      device.destroy_image(self.raw);

      allocator.free(device, self.memory);
    }
  }
}

pub(crate) fn create_view(
  device: &Device,
  raw_image: &RawTexture,
  format: TextureFormat,
) -> RawTextureView {
  unsafe {
    device
      .create_image_view(
        &raw_image,
        gfx_hal::image::ViewKind::D2,
        format,
        gfx_hal::format::Swizzle::NO,
        gfx_hal::image::SubresourceRange {
          aspects: gfx_hal::format::Aspects::COLOR,
          levels: 0..1,
          layers: 0..1,
        },
      )
      .expect("Could not create image view")
  }
}
