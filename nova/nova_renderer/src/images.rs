// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::format::Format as DeviceImageFormat;
pub use gfx_hal::image::Access as DeviceImageAccess;
pub use gfx_hal::image::CreationError as CreateImageError;
pub use gfx_hal::image::Layout as DeviceImageLayout;
pub use gfx_hal::image::ViewError as CreateViewError;

use crate::alloc::{Allocator, Memory, MemoryBindError, MemoryKind};
use crate::pipeline::MemoryBarrier;
use crate::{Backend, Device, DeviceExt};
use nova_core::quick_error;
use nova_math::Size;

pub type RawDeviceImage = <Backend as gfx_hal::Backend>::Image;
pub type RawDeviceImageView = <Backend as gfx_hal::Backend>::ImageView;

#[derive(Debug)]
pub struct DeviceImage {
  pub(crate) raw: RawDeviceImage,
  pub(crate) raw_view: RawDeviceImageView,
  memory: Memory,
  size: Size<u32>,
}

impl DeviceImage {
  pub fn new(
    device: &Device,
    allocator: &mut Allocator,
    size: Size<u32>,
    format: DeviceImageFormat,
  ) -> Result<DeviceImage, NewDeviceImageError> {
    let mut raw = unsafe {
      device.create_image(
        gfx_hal::image::Kind::D2(size.width, size.height, 1, 1),
        1,
        format,
        gfx_hal::image::Tiling::Optimal,
        gfx_hal::image::Usage::TRANSFER_DST | gfx_hal::image::Usage::SAMPLED,
        gfx_hal::image::ViewCapabilities::empty(),
      )?
    };

    let requirements = unsafe { device.get_image_requirements(&raw) };
    let memory = allocator.alloc(device, MemoryKind::Gpu, requirements);

    unsafe { device.bind_image_memory(&memory, 0, &mut raw)? };

    let raw_view = create_raw_view(device, &raw, format)?;

    Ok(DeviceImage {
      raw,
      raw_view,
      memory,
      size,
    })
  }

  pub fn size(&self) -> Size<u32> {
    self.size
  }

  pub(crate) fn memory_barrier(
    &self,
    access_change: (DeviceImageAccess, DeviceImageAccess),
    layout_change: (DeviceImageLayout, DeviceImageLayout),
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
    }

    allocator.free(device, self.memory);
  }
}

pub(crate) fn create_raw_view(
  device: &Device,
  raw_image: &RawDeviceImage,
  format: DeviceImageFormat,
) -> Result<RawDeviceImageView, CreateViewError> {
  unsafe {
    device.create_image_view(
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
  }
}

quick_error! {
  #[derive(Debug)]
  pub enum NewDeviceImageError {
    CreateImage(err: CreateImageError) {
      from()
      display("could not create device image: {}", err)
    }
    BindMemmory(err: MemoryBindError) {
      from()
      display("could not bind device image memory: {}", err)
    }
    CreateView(err: CreateViewError) {
      from()
      display("could not create device image view: {}", err)
    }
  }
}
