pub use gfx_hal::buffer::Usage as BufferUsage;

use super::{Allocator, Device};
use crate::graphics::backend::{self, Backend};
use crate::graphics::hal::prelude::*;
use crate::utils::Droppable;
use gfx_memory::{Block, Factory};
use std::mem;
use std::sync::Arc;

type Allocation = <Allocator as Factory<Backend>>::Buffer;

pub struct Buffer<T> {
  inner: Droppable<Allocation>,
  size: u64,
  device: Arc<Device>,
  _marker: std::marker::PhantomData<T>,
}

impl<T: Copy> Buffer<T> {
  pub fn new(device: &Arc<Device>, len: usize, usage: BufferUsage) -> Self {
    let size = mem::size_of::<T>() as u64 * len as u64;

    let inner = device
      .allocator()
      .create_buffer(
        device.raw(),
        (
          gfx_memory::Type::General,
          gfx_hal::memory::Properties::CPU_VISIBLE,
        ),
        size,
        usage,
      )
      .expect("could not allocate buffer");

    Buffer {
      device: device.clone(),
      inner: inner.into(),
      size,
      _marker: std::marker::PhantomData,
    }
  }

  pub fn write(&mut self, values: &[T]) {
    let device = self.device.raw();
    let memory = self.inner.memory();
    let range = self.inner.range();

    let mut dest = device
      .acquire_mapping_writer::<T>(&memory, range.start..range.start + self.size)
      .expect("could not acquire mapping writer");

    dest.copy_from_slice(values);

    device.release_mapping_writer(dest).expect("out of memory");
  }

  pub fn raw(&self) -> &backend::Buffer {
    self.inner.raw()
  }
}

impl<T> Drop for Buffer<T> {
  fn drop(&mut self) {
    if let Some(inner) = self.inner.take() {
      self
        .device
        .allocator()
        .destroy_buffer(self.device.raw(), inner);
    }
  }
}
