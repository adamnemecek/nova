// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::buffer::Usage;

use super::device::{self, Device};
use crate::graphics::backend::{self, Backend};
use crate::graphics::hal::prelude::*;
use crate::utils::Droppable;
use gfx_memory::{Block, Factory};
use std::mem;
use std::sync::Arc;

/// Value returned from the device allocator's `create_buffer` method.
type Allocation = <device::Allocator as Factory<Backend>>::Buffer;

/// A buffer of device-accessible memory.
pub struct Buffer<T> {
  /// Device the buffer was created with.
  device: Arc<Device>,
  /// Raw backend buffer and memory allocated by the device.
  inner: Droppable<Allocation>,
  /// Size of the buffer in bytes.
  size: u64,
  /// A phantom data marker so that buffers can be generic on their contents.
  _marker: std::marker::PhantomData<T>,
}

impl<T: Copy> Buffer<T> {
  /// Creates a new buffer large enough for `len` elements.
  pub fn new(device: &Arc<Device>, len: usize, usage: Usage) -> Self {
    let size = mem::size_of::<T>() as u64 * len as u64;

    let inner = device
      .allocator()
      .create_buffer(
        device.raw(),
        (
          // TODO: Other kinds of buffers.
          gfx_memory::Type::General,
          hal::memory::Properties::CPU_VISIBLE,
        ),
        size,
        usage,
      )
      .expect("Could not allocate buffer");

    Buffer {
      device: device.clone(),
      inner: inner.into(),
      size,
      _marker: std::marker::PhantomData,
    }
  }

  /// Writes new contents to the buffer.
  ///
  /// Panics if the given slice is not the same length as the buffer.
  pub fn write(&mut self, values: &[T]) {
    let device = self.device.raw();
    let memory = self.inner.memory();
    let range = self.inner.range();

    let mut dest = device
      .acquire_mapping_writer::<T>(&memory, range.start..range.start + self.size)
      .expect("Could not acquire mapping writer");

    dest.copy_from_slice(values);

    device
      .release_mapping_writer(dest)
      .expect("Could not release mapping writer");
  }
}

// Implement `AsRef` to expose the raw backend buffer.
impl<T> AsRef<backend::Buffer> for Buffer<T> {
  fn as_ref(&self) -> &backend::Buffer {
    self.inner.raw()
  }
}

// Implement `Drop` to destroy the buffer with the device allocator.
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
