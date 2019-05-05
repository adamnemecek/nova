// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::gpu::Gpu;
use crate::Backend;
use gfx_hal::memory::Requirements as MemoryRequirements;
use gfx_hal::{Device as _, MemoryProperties};

pub type Memory = <Backend as gfx_hal::Backend>::Memory;

#[derive(Debug)]
pub enum MemoryKind {
  Gpu,
  Cpu,
}

#[derive(Debug)]
pub struct Allocator {
  memory_properties: MemoryProperties,
}

impl Allocator {
  pub fn new(memory_properties: MemoryProperties) -> Self {
    Allocator { memory_properties }
  }

  pub fn alloc(&mut self, gpu: &Gpu, kind: MemoryKind, requirements: MemoryRequirements) -> Memory {
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
      .expect("Could not find approprate memory type");

    unsafe {
      gpu
        .device
        .allocate_memory(type_id, requirements.size)
        .expect("Could not allocate memory")
    }
  }

  pub fn free(&mut self, gpu: &Gpu, memory: Memory) {
    unsafe { gpu.device.free_memory(memory) };
  }
}
