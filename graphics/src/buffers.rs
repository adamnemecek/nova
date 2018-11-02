use super::backend;
use super::Context;
pub use gfx_hal::buffer::Usage as BufferUsage;
use gfx_hal::Device;
use std::sync::Arc;

pub struct Buffer<T> {
  size: u64,
  inner: Option<Inner>,
  context: Arc<Context>,
  _marker: std::marker::PhantomData<T>,
}

struct Inner {
  raw: backend::Buffer,
  memory: backend::Memory,
}

impl<T: Copy> Buffer<T> {
  pub fn new(context: &Arc<Context>, size: usize, usage: BufferUsage) -> Self {
    let device = &context.device;

    let size = std::mem::size_of::<T>() as u64 * size as u64;

    let unbound = device
      .create_buffer(size, usage)
      .expect("could not create buffer");

    let requirements = device.get_buffer_requirements(&unbound);

    let upload_type = context
      .memory_properties
      .memory_types
      .iter()
      .enumerate()
      .find(|(id, ty)| {
        let supported = requirements.type_mask & (1_u64 << id) != 0;

        supported && ty
          .properties
          .contains(gfx_hal::memory::Properties::CPU_VISIBLE)
      }).map(|(id, _ty)| gfx_hal::MemoryTypeId(id))
      .expect("could not find approprate vertex buffer memory type");

    let memory = device
      .allocate_memory(upload_type, requirements.size)
      .unwrap();

    let buffer = device
      .bind_buffer_memory(&memory, 0, unbound)
      .expect("could not bind buffer memory");

    Buffer {
      context: context.clone(),
      size,
      inner: Some(Inner {
        raw: buffer,
        memory,
      }),
      _marker: std::marker::PhantomData,
    }
  }

  pub fn write(&self, values: &[T]) {
    let device = &self.context.device;
    let inner = self.inner.as_ref().expect("buffer inner was destroyed");

    let mut dest = device
      .acquire_mapping_writer::<T>(&inner.memory, 0..self.size)
      .unwrap();

    dest.copy_from_slice(values);

    device.release_mapping_writer(dest);
  }

  pub fn raw(&self) -> &backend::Buffer {
    let inner = self.inner.as_ref().expect("buffer inner was destroyed");

    &inner.raw
  }
}

impl<T> Drop for Buffer<T> {
  fn drop(&mut self) {
    let device = &self.context.device;

    if let Some(inner) = self.inner.take() {
      device.destroy_buffer(inner.raw);
      device.free_memory(inner.memory);
    }
  }
}
