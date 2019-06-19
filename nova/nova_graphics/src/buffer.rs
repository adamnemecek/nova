// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use std::slice::SliceIndex;

/// One of the possible kinds of `Buffer`.
#[derive(Debug, Clone, Copy)]
pub enum BufferKind {
  /// Host-mapped memory for use as a transfer source.
  Staging,
  /// Contains vertex data.
  Vertex,
  /// Contains index data.
  Index,
}

/// Device-local buffer of data.
///
/// If the buffer has `BufferKind::Staging`, then its data can be directly
/// read from and written to with the index operator.
pub struct Buffer<T> {
  context: Context,
  _memory: MemoryBlock,
  buffer: Option<backend::Buffer>,
  mapped: Option<*mut T>,
  len: u64,
  byte_len: u64,
}

#[allow(clippy::len_without_is_empty)]
impl<T: Copy> Buffer<T> {
  /// Allocates a new buffer of the given length.
  pub fn new(context: &Context, kind: BufferKind, len: u64) -> Result<Self, BufferCreationError> {
    let byte_len = len
      .checked_mul(mem::size_of::<T>() as u64)
      .expect("requested buffer size is too large");

    let memory_kind = match kind {
      BufferKind::Staging => MemoryKind::HostMapped,
      _ => MemoryKind::DeviceLocal,
    };

    let usage = match kind {
      BufferKind::Vertex => gfx_hal::buffer::Usage::VERTEX | gfx_hal::buffer::Usage::TRANSFER_DST,
      BufferKind::Index => gfx_hal::buffer::Usage::INDEX | gfx_hal::buffer::Usage::TRANSFER_DST,
      BufferKind::Staging => gfx_hal::buffer::Usage::TRANSFER_SRC,
    };

    let mut buffer = unsafe { context.device.create_buffer(byte_len, usage)? };
    let requirements = unsafe { context.device.get_buffer_requirements(&buffer) };
    let memory = context.allocator().alloc(memory_kind, requirements)?;

    unsafe {
      context
        .device
        .bind_buffer_memory(memory.as_backend(), 0, &mut buffer)?;
    }

    let mapped = match kind {
      BufferKind::Staging => Some(unsafe {
        context
          .device
          .map_memory(memory.as_backend(), 0..byte_len as u64)? as *mut T
      }),

      _ => None,
    };

    Ok(Self {
      context: context.clone(),
      _memory: memory,
      buffer: Some(buffer),
      mapped,
      len,
      byte_len,
    })
  }
}

impl<T> Buffer<T> {
  /// Returns the length of the buffer.
  ///
  /// This is the number of `T` values that fit in the buffer.
  pub fn len(&self) -> u64 {
    self.len
  }

  /// Returns the length of the buffer in bytes.
  pub fn byte_len(&self) -> u64 {
    self.byte_len
  }

  /// Returns a reference to the underlying backend buffer.
  pub(crate) fn as_backend(&self) -> &backend::Buffer {
    self.buffer.as_ref().unwrap()
  }
}

impl<T> Drop for Buffer<T> {
  fn drop(&mut self) {
    unsafe {
      self
        .context
        .device
        .destroy_buffer(self.buffer.take().unwrap());
    }
  }
}

impl<T, I: SliceIndex<[T]>> ops::Index<I> for Buffer<T> {
  type Output = I::Output;

  fn index(&self, index: I) -> &Self::Output {
    let mapped = self.mapped.expect("cannot index a non-staging buffer");
    let slice = unsafe { std::slice::from_raw_parts_mut(mapped, self.byte_len as usize) };

    &slice[index]
  }
}

impl<T, I: SliceIndex<[T]>> ops::IndexMut<I> for Buffer<T> {
  fn index_mut(&mut self, index: I) -> &mut I::Output {
    let mapped = self.mapped.expect("cannot index a non-staging buffer");

    let slice = unsafe { std::slice::from_raw_parts_mut(mapped, self.byte_len as usize) };

    &mut slice[index]
  }
}

// An error that occurred during the creation of a new `Buffer`.
#[derive(Debug)]
pub enum BufferCreationError {
  /// Out of either device or host memory to create resources with.
  OutOfMemory,
  /// The requested `BufferKind` is not supported.
  KindNotSupported,
  /// Failed to allocate a memory block for the buffer.
  AllocationFailed(AllocationError),
}

impl std::error::Error for BufferCreationError {}

impl From<gfx_hal::buffer::CreationError> for BufferCreationError {
  fn from(error: gfx_hal::buffer::CreationError) -> Self {
    match error {
      gfx_hal::buffer::CreationError::OutOfMemory(_) => BufferCreationError::OutOfMemory,
      gfx_hal::buffer::CreationError::UnsupportedUsage { .. } => {
        BufferCreationError::KindNotSupported
      }
    }
  }
}

impl From<AllocationError> for BufferCreationError {
  fn from(error: AllocationError) -> Self {
    BufferCreationError::AllocationFailed(error)
  }
}

impl From<gfx_hal::device::BindError> for BufferCreationError {
  fn from(error: gfx_hal::device::BindError) -> Self {
    match error {
      gfx_hal::device::BindError::OutOfMemory(_) => BufferCreationError::OutOfMemory,
      error => panic!("failed to bind buffer memory: {}", error),
    }
  }
}

impl From<gfx_hal::mapping::Error> for BufferCreationError {
  fn from(error: gfx_hal::mapping::Error) -> Self {
    match error {
      gfx_hal::mapping::Error::OutOfMemory(_) => BufferCreationError::OutOfMemory,
      error => panic!("failed to map staging buffer memory: {}", error),
    }
  }
}

impl fmt::Display for BufferCreationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      BufferCreationError::OutOfMemory => write!(f, "out of memory"),
      BufferCreationError::KindNotSupported => write!(f, "requested memory kind not supported"),
      BufferCreationError::AllocationFailed(error) => {
        write!(f, "memory block allocation failed: {}", error)
      }
    }
  }
}
