pub use gfx_hal::buffer::Usage as BufferUsage;

use super::*;
use std::mem;
use std::sync::Arc;

pub struct Buffer<T> {
  size: u64,
  raw: Option<backend::Buffer>,
  memory: Option<backend::Memory>,
  device: Arc<Device>,
  _marker: std::marker::PhantomData<T>,
}

impl<T: Copy> Buffer<T> {
  pub fn new(device: &Arc<Device>, size: usize, usage: BufferUsage) -> Self {
    let size = mem::size_of::<T>() as u64 * size as u64;

    let unbound = device
      .raw
      .create_buffer(size, usage)
      .expect("could not create buffer");

    let requirements = device.raw.get_buffer_requirements(&unbound);

    let upload_type = device
      .memory_properties
      .memory_types
      .iter()
      .enumerate()
      .find(|(id, ty)| {
        let supported = requirements.type_mask & (1_u64 << id) != 0;

        supported
          && ty
            .properties
            .contains(gfx_hal::memory::Properties::CPU_VISIBLE)
      })
      .map(|(id, _ty)| gfx_hal::MemoryTypeId(id))
      .expect("could not find approprate vertex buffer memory type");

    let memory = device
      .raw
      .allocate_memory(upload_type, requirements.size)
      .unwrap();

    let buffer = device
      .raw
      .bind_buffer_memory(&memory, 0, unbound)
      .expect("could not bind buffer memory");

    Buffer {
      device: device.clone(),
      size,
      memory: Some(memory),
      raw: Some(buffer),
      _marker: std::marker::PhantomData,
    }
  }

  pub fn write(&mut self, values: &[T]) {
    let device = &self.device.raw;
    let memory = self.memory.as_ref().expect("memory was destroyed");

    let mut dest = device
      .acquire_mapping_writer::<T>(&memory, 0..self.size)
      .unwrap();

    dest.copy_from_slice(values);

    device.release_mapping_writer(dest).expect("out of memory");
  }

  pub fn raw(&self) -> &backend::Buffer {
    self.raw.as_ref().expect("buffer was destroyed")
  }
}

impl<T> Drop for Buffer<T> {
  fn drop(&mut self) {
    let device = &self.device.raw;

    if let Some(memory) = self.memory.take() {
      device.free_memory(memory);
    }

    if let Some(buffer) = self.raw.take() {
      device.destroy_buffer(buffer);
    }
  }
}
