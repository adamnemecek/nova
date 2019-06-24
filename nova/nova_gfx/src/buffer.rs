// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Buffer of data on the graphics device.
pub struct Buffer {
  context: Context,
  buffer: Option<backend::Buffer>,
  mapped: Option<*mut u8>,
  len: u64,
  _memory: MemoryBlock,
}

// TODO: Is there a better way to avoid that `*mut u8` is not `Send`?
unsafe impl Send for Buffer {}

impl Buffer {
  /// Allocates a new buffer of the given length.
  pub fn new(context: &Context, kind: BufferKind, len: u64) -> Result<Self, BufferCreationError> {
    let memory_kind = match kind {
      BufferKind::Staging => MemoryKind::HostMapped,
      _ => MemoryKind::DeviceLocal,
    };

    let usage = match kind {
      BufferKind::Vertex => gfx_hal::buffer::Usage::VERTEX | gfx_hal::buffer::Usage::TRANSFER_DST,
      BufferKind::Index => gfx_hal::buffer::Usage::INDEX | gfx_hal::buffer::Usage::TRANSFER_DST,
      BufferKind::Staging => gfx_hal::buffer::Usage::TRANSFER_SRC,
      BufferKind::Uniform => gfx_hal::buffer::Usage::UNIFORM | gfx_hal::buffer::Usage::TRANSFER_DST,
    };

    let mut buffer = unsafe { context.device.create_buffer(len, usage)? };
    let requirements = unsafe { context.device.get_buffer_requirements(&buffer) };
    let memory = context.allocator().alloc(memory_kind, requirements)?;

    unsafe {
      context
        .device
        .bind_buffer_memory(memory.as_backend(), 0, &mut buffer)?;
    }

    let mapped = match kind {
      BufferKind::Staging => {
        Some(unsafe { context.device.map_memory(memory.as_backend(), 0..len)? })
      }

      _ => None,
    };

    Ok(Self {
      context: context.clone(),
      buffer: Some(buffer),
      mapped,
      len,
      _memory: memory,
    })
  }
}

#[allow(clippy::len_without_is_empty)]
impl Buffer {
  /// Returns the length of the buffer bytes.
  pub fn len(&self) -> u64 {
    self.len
  }

  pub fn slice_as_ref<T: Copy>(&self, bounds: impl ops::RangeBounds<u64>) -> &[T] {
    let mapped = unsafe {
      slice::from_raw_parts(
        self
          .mapped
          .expect("cannot get a direct reference to a non-staging buffer"),
        self.len as usize,
      )
    };

    let slice = &mapped[clamp_buffer_range_usize(&self, bounds)];

    unsafe {
      slice::from_raw_parts(
        &slice[0] as *const u8 as *const T,
        slice.len() / mem::size_of::<T>(),
      )
    }
  }

  pub fn slice_as_mut<T: Copy>(&mut self, bounds: impl ops::RangeBounds<u64>) -> &mut [T] {
    let mapped = unsafe {
      slice::from_raw_parts_mut(
        self
          .mapped
          .expect("cannot get a direct reference to a non-staging buffer"),
        self.len as usize,
      )
    };

    let slice = &mut mapped[clamp_buffer_range_usize(&self, bounds)];

    unsafe {
      slice::from_raw_parts_mut(
        &mut slice[0] as *mut u8 as *mut T,
        slice.len() / mem::size_of::<T>(),
      )
    }
  }

  /// Returns a reference to the underlying backend buffer.
  pub(crate) fn as_backend(&self) -> &backend::Buffer {
    self.buffer.as_ref().unwrap()
  }
}

impl Drop for Buffer {
  fn drop(&mut self) {
    unsafe {
      self
        .context
        .device
        .destroy_buffer(self.buffer.take().unwrap());
    }
  }
}

impl<T: Copy> AsRef<[T]> for Buffer {
  fn as_ref(&self) -> &[T] {
    self.slice_as_ref(..)
  }
}

impl<T: Copy> AsMut<[T]> for Buffer {
  fn as_mut(&mut self) -> &mut [T] {
    self.slice_as_mut(..)
  }
}

/// One of the possible kinds of `Buffer`.
#[derive(Debug, Clone, Copy)]
pub enum BufferKind {
  /// Contains temporary data for transferring to images and buffers of other
  /// kinds. Staging buffers are mapped into host memory for direct access.
  Staging,
  /// Contains vertex data.
  Vertex,
  /// Contains index data.
  Index,
  /// Contains data readable by shaders.
  Uniform,
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

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct BufferRange {
  pub start: u64,
  pub len: u64,
}

pub(crate) fn clamp_buffer_range_usize(
  buffer: &Buffer,
  bounds: impl ops::RangeBounds<u64>,
) -> ops::Range<usize> {
  let start = match bounds.start_bound() {
    ops::Bound::Unbounded => 0,
    ops::Bound::Included(i) => *i,
    ops::Bound::Excluded(i) => *i + 1,
  };

  let end = match bounds.start_bound() {
    ops::Bound::Unbounded => buffer.len(),
    ops::Bound::Included(i) => *i,
    ops::Bound::Excluded(i) => *i - 1,
  };

  (start as usize)..(end as usize)
}
