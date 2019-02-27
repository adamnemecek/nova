use super::alloc::{Allocator, Memory, MemoryKind};
use super::device::{Device, DeviceExt};
use super::Backend;
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

type RawBuffer = <Backend as gfx_hal::Backend>::Buffer;

#[derive(Debug)]
pub struct Buffer {
  pub(crate) raw: RawBuffer,
  memory: Memory,
  size: usize,
  mapped: Option<*mut u8>,
}

impl Buffer {
  pub fn new(device: &Device, allocator: &mut Allocator, kind: BufferKind, size: usize) -> Self {
    let usage = match kind {
      BufferKind::Vertex => gfx_hal::buffer::Usage::VERTEX,
      BufferKind::Index => gfx_hal::buffer::Usage::INDEX,
      BufferKind::Staging => gfx_hal::buffer::Usage::TRANSFER_SRC,
    };

    let mut raw = unsafe {
      device
        .create_buffer(size as u64, usage)
        .expect("Could not create buffer")
    };

    let memory_kind = match kind {
      BufferKind::Staging => MemoryKind::Cpu,
      _ => MemoryKind::Gpu,
    };

    let memory = allocator.alloc(&device, memory_kind, unsafe {
      device.get_buffer_requirements(&raw)
    });

    unsafe {
      device
        .bind_buffer_memory(&memory, 0, &mut raw)
        .expect("Could not bind buffer memory");
    }

    let mapped = unsafe {
      device
        .map_memory(&memory, 0..(size as u64))
        .expect("Could not map memory")
    };

    Buffer {
      raw,
      memory,
      size,
      mapped: Some(mapped),
    }
  }

  pub fn destroy(self, device: &Device, allocator: &mut Allocator) {
    unsafe {
      if self.mapped.is_some() {
        device.unmap_memory(&self.memory);
      }

      allocator.free(device, self.memory);

      device.destroy_buffer(self.raw);
    }
  }
}

impl<I: SliceIndex<[u8]>> Index<I> for Buffer {
  type Output = I::Output;

  fn index(&self, index: I) -> &Self::Output {
    let mapped = self.mapped.expect("Buffer memory is not mapped.");
    let slice = unsafe { std::slice::from_raw_parts_mut(mapped, self.size) };

    &slice[index]
  }
}

impl<I: SliceIndex<[u8]>> IndexMut<I> for Buffer {
  fn index_mut(&mut self, index: I) -> &mut I::Output {
    let mapped = self.mapped.expect("Buffer memory is not mapped.");
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
