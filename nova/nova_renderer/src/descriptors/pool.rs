// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::DescriptorPool as RawDescriptorPoolExt;

use super::{Descriptor, DescriptorLayout, DescriptorSet};
use crate::{Backend, Device, DeviceExt};
use gfx_hal::pso::DescriptorRangeDesc;
use std::iter;

pub type RawDescriptorPool = <Backend as gfx_hal::Backend>::DescriptorPool;

#[derive(Debug)]
pub struct DescriptorPool {
  pub(crate) raw: RawDescriptorPool,
  layout: DescriptorLayout,
}

impl DescriptorPool {
  pub fn new(device: &Device, layout: DescriptorLayout) -> DescriptorPool {
    let raw = unsafe {
      device
        .create_descriptor_pool(
          4096,
          layout.kinds().map(|kind| DescriptorRangeDesc {
            ty: kind.ty(),
            count: 1024,
          }),
        )
        .expect("Could not create descriptor pool")
    };

    DescriptorPool { raw, layout }
  }

  pub fn layout(&self) -> &DescriptorLayout {
    &self.layout
  }

  pub fn alloc(&mut self, device: &Device, descriptors: &[Descriptor]) -> DescriptorSet {
    let set = unsafe {
      self
        .raw
        .allocate_set(self.layout.raw())
        .expect("Could not allocate descriptor set")
    };

    unsafe {
      device.write_descriptor_sets(iter::once(gfx_hal::pso::DescriptorSetWrite {
        set: &set,
        binding: 0,
        array_offset: 0,
        descriptors: descriptors.iter().map(gfx_hal::pso::Descriptor::from),
      }));
    }

    set
  }

  pub fn destroy(self, device: &Device) {
    unsafe {
      device.destroy_descriptor_pool(self.raw);
    }

    self.layout.destroy(device);
  }
}
