// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::device::{Device, DeviceExt, PhysicalDevice, PhysicalDeviceExt};
use super::Backend;
use gfx_hal::memory::Requirements as MemoryRequirements;
use gfx_hal::MemoryProperties;

pub type Memory = <Backend as gfx_hal::Backend>::Memory;

#[derive(Debug)]
pub struct Allocator {
  memory_properties: MemoryProperties,
}

impl Allocator {
  pub fn new(physical_device: &PhysicalDevice) -> Self {
    Allocator {
      memory_properties: physical_device.memory_properties(),
    }
  }

  pub fn alloc(
    &mut self,
    device: &Device,
    kind: MemoryKind,
    requirements: MemoryRequirements,
  ) -> Memory {
    let properties = match kind {
      MemoryKind::Cpu => {
        gfx_hal::memory::Properties::CPU_VISIBLE | gfx_hal::memory::Properties::COHERENT
      }
      MemoryKind::Gpu => gfx_hal::memory::Properties::DEVICE_LOCAL,
    };

    let type_id = self
      .memory_properties
      .memory_types
      .iter()
      .enumerate()
      .find(|(id, ty)| {
        let supported = requirements.type_mask & (1_u64 << id) != 0;

        supported && ty.properties.contains(properties)
      })
      .map(|(id, _ty)| gfx_hal::MemoryTypeId(id))
      .expect("could not find approprate memory type");

    unsafe {
      device
        .allocate_memory(type_id, requirements.size)
        .expect("could not allocate memory")
    }
  }

  pub fn free(&mut self, device: &Device, memory: Memory) {
    unsafe {
      device.free_memory(memory);
    }
  }
}

#[derive(Debug)]
pub enum MemoryKind {
  Gpu,
  Cpu,
}
