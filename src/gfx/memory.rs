// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::memory::{Properties as MemoryProperties, Requirements as MemoryRequirements};

use super::*;

pub struct Memory {
  heaps: Vec<MemoryHeap>,
}

impl Memory {
  pub fn new(adapter: &backend::Adapter) -> Self {
    let properties = adapter.physical_device.memory_properties();

    let heaps = properties.memory_heaps.into_iter().map(MemoryHeap::new).collect();

    let mut memory = Memory { heaps };
    for (id, memory_type) in properties.memory_types.into_iter().enumerate() {
      memory.heaps[memory_type.heap_index]
        .memory_types
        .push(MemoryType { id: gfx_hal::MemoryTypeId(id), properties: memory_type.properties })
    }

    memory
  }
}

struct MemoryHeap {
  size: u64,
  memory_types: Vec<MemoryType>,
}

impl MemoryHeap {
  fn new(size: u64) -> Self {
    Self { size, memory_types: Vec::new() }
  }
}

pub struct MemoryType {
  pub id: gfx_hal::MemoryTypeId,
  pub properties: MemoryProperties,
}

pub struct MemoryBlock {
  context: Arc<Context>,
  memory: Expect<backend::Memory>,
  heap_index: usize,
  type_index: usize,
  size: u64,
}

impl MemoryBlock {
  pub fn as_backend(&self) -> &backend::Memory {
    &self.memory
  }
}

impl Drop for MemoryBlock {
  fn drop(&mut self) {
    unsafe {
      self.context.device().free_memory(self.memory.take());
    }
  }
}

pub fn alloc(
  context: &Arc<Context>,
  requirements: MemoryRequirements,
  properties: MemoryProperties,
) -> Result<MemoryBlock, AllocationError> {
  let device = context.device();
  let size = requirements.size;

  let mut err: Option<AllocationError> = None;

  for (heap_index, heap) in context.memory().heaps.iter().enumerate() {
    for (type_index, memory_type) in heap.memory_types.iter().enumerate() {
      let result = unsafe { device.allocate_memory(memory_type.id, size) };

      match result {
        Ok(memory) => {
          return Ok(MemoryBlock {
            context: context.clone(),
            memory: memory.into(),
            heap_index,
            type_index,
            size,
          });
        }

        Err(e) => {
          err = Some(e.into());
        }
      }
    }
  }

  Err(err.unwrap_or(AllocationError::NoSuitableMemoryType))
}

pub fn free(block: MemoryBlock) {
  drop(block);
}

/// An error that occurred while allocating a `MemoryBlock`.
#[derive(Debug, Clone, Copy)]
pub enum AllocationError {
  /// None of the available memory types is suitable for the request.
  NoSuitableMemoryType,
  /// Out of either host or device memory.
  OutOfMemory,
  /// Too many blocks of memory have been allocated.
  TooManyBlocks,
}

impl std::error::Error for AllocationError {}

impl fmt::Display for AllocationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      AllocationError::NoSuitableMemoryType => write!(f, "no suitable memory type"),
      AllocationError::OutOfMemory => write!(f, "out of memory"),
      AllocationError::TooManyBlocks => write!(f, "too many memory blocks allocated"),
    }
  }
}

// Impl `From` to convert from backend allocation errors.
impl From<gfx_hal::device::AllocationError> for AllocationError {
  fn from(error: gfx_hal::device::AllocationError) -> Self {
    match error {
      gfx_hal::device::AllocationError::OutOfMemory(_) => AllocationError::OutOfMemory,
      gfx_hal::device::AllocationError::TooManyObjects => AllocationError::TooManyBlocks,
    }
  }
}
