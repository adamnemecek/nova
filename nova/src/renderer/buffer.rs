use super::alloc::{Allocator, Memory, MemoryKind};
use super::device::{Device, DeviceExt};
use super::Backend;

type RawBuffer = <Backend as gfx_hal::Backend>::Buffer;

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

  pub fn bytes_mut(&mut self) -> &mut [u8] {
    let mapped = self.mapped.expect("Buffer memory is not mapped.");

    unsafe { std::slice::from_raw_parts_mut(mapped, self.size) }
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

#[derive(Debug)]
pub enum BufferKind {
  Vertex,
  Index,
  Staging,
}
