// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::gpu::alloc::{Allocator, Memory, MemoryKind};
use crate::gpu::Gpu;
use crate::Backend;
use gfx_hal::Device as _;
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

type HalBuffer = <Backend as gfx_hal::Backend>::Buffer;

#[derive(Debug)]
pub struct Buffer {
  buffer: HalBuffer,
  memory: Memory,
  size: usize,
  mapped: Option<*mut u8>,
}

impl Buffer {
  pub fn new(gpu: &Gpu, allocator: &mut Allocator, kind: BufferKind, size: usize) -> Self {
    let usage = match kind {
      BufferKind::Vertex => gfx_hal::buffer::Usage::VERTEX,
      BufferKind::Index => gfx_hal::buffer::Usage::INDEX,
      BufferKind::Staging => gfx_hal::buffer::Usage::TRANSFER_SRC,
    };

    let mut buffer = unsafe {
      gpu
        .device
        .create_buffer(size as u64, usage)
        .expect("Could not create buffer")
    };

    let memory_kind = match kind {
      BufferKind::Staging => MemoryKind::Cpu,
      _ => MemoryKind::Gpu,
    };

    let memory = allocator.alloc(&gpu, memory_kind, unsafe {
      gpu.device.get_buffer_requirements(&buffer)
    });

    unsafe {
      gpu
        .device
        .bind_buffer_memory(&memory, 0, &mut buffer)
        .expect("Could not bind buffer memory");
    }

    let mapped = match kind {
      BufferKind::Staging => unsafe {
        gpu
          .device
          .map_memory(&memory, 0..(size as u64))
          .expect("Could not map memory")
          .into()
      },

      _ => None,
    };

    Buffer {
      buffer,
      memory,
      size,
      mapped,
    }
  }

  pub fn destroy(self, gpu: &Gpu, allocator: &mut Allocator) {
    if self.mapped.is_some() {
      unsafe { gpu.device.unmap_memory(&self.memory) };
    }

    allocator.free(gpu, self.memory);

    unsafe { gpu.device.destroy_buffer(self.buffer) };
  }
}

impl<I: SliceIndex<[u8]>> Index<I> for Buffer {
  type Output = I::Output;

  fn index(&self, index: I) -> &Self::Output {
    let mapped = self.mapped.expect("Only staging buffers can be read from.");
    let slice = unsafe { std::slice::from_raw_parts_mut(mapped, self.size) };

    &slice[index]
  }
}

impl<I: SliceIndex<[u8]>> IndexMut<I> for Buffer {
  fn index_mut(&mut self, index: I) -> &mut I::Output {
    let mapped = self
      .mapped
      .expect("Only staging buffers can be written to.");

    let slice = unsafe { std::slice::from_raw_parts_mut(mapped, self.size) };

    &mut slice[index]
  }
}

#[derive(Debug)]
pub enum BufferKind {
  Vertex,
  Index,
  Staging,
}
