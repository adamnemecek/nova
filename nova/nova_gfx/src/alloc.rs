// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::backend;
use crate::Context;
use gfx_hal::Device as _;
use std::fmt;

/// One of the possible kinds of memory.
#[derive(Debug)]
pub enum MemoryKind {
  /// Memory accessible by both the graphics device and the host.
  HostMapped,
  /// Memory local to and only accessible by the graphics device.
  DeviceLocal,
}

pub struct Memory {
  properties: gfx_hal::MemoryProperties,
}

impl Memory {
  pub fn new(properties: gfx_hal::MemoryProperties) -> Memory {
    Self { properties }
  }
}

pub struct MemoryBlock {
  context: Context,
  memory: Option<backend::Memory>,
}

impl MemoryBlock {
  pub(crate) fn as_backend(&self) -> &backend::Memory {
    self.memory.as_ref().unwrap()
  }
}

impl Drop for MemoryBlock {
  fn drop(&mut self) {
    if let Some(memory) = self.memory.take() {
      self.context.allocator().free(MemoryBlock {
        context: self.context.clone(),
        memory: Some(memory),
      });
    }
  }
}

pub struct Allocator<'a> {
  context: &'a Context,
}

impl<'a> Allocator<'a> {
  pub fn new(context: &'a Context) -> Self {
    Allocator { context }
  }

  pub(crate) fn alloc(
    &mut self,
    kind: MemoryKind,
    requirements: gfx_hal::memory::Requirements,
  ) -> Result<MemoryBlock, AllocationError> {
    let properties = match kind {
      MemoryKind::HostMapped => {
        gfx_hal::memory::Properties::CPU_VISIBLE | gfx_hal::memory::Properties::COHERENT
      }

      MemoryKind::DeviceLocal => gfx_hal::memory::Properties::DEVICE_LOCAL,
    };

    let type_id = self
      .context
      .memory
      .properties
      .memory_types
      .iter()
      .enumerate()
      .find(|(id, ty)| {
        let supported = requirements.type_mask & (1_u64 << id) != 0;

        supported && ty.properties.contains(properties)
      })
      .map(|(id, _ty)| gfx_hal::MemoryTypeId(id))
      .ok_or(AllocationError::NoSuitableMemoryType)?;

    let memory = unsafe {
      self
        .context
        .device
        .allocate_memory(type_id, requirements.size)?
    };

    Ok(MemoryBlock {
      memory: Some(memory),
      context: self.context.clone(),
    })
  }

  pub(crate) fn free(&self, mut block: MemoryBlock) {
    unsafe {
      self
        .context
        .device
        .free_memory(block.memory.take().unwrap());
    }
  }
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

impl From<gfx_hal::device::AllocationError> for AllocationError {
  fn from(error: gfx_hal::device::AllocationError) -> Self {
    match error {
      gfx_hal::device::AllocationError::OutOfMemory(_) => AllocationError::OutOfMemory,
      gfx_hal::device::AllocationError::TooManyObjects => AllocationError::TooManyBlocks,
    }
  }
}

impl fmt::Display for AllocationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      AllocationError::NoSuitableMemoryType => write!(f, "no suitable memory type"),
      AllocationError::OutOfMemory => write!(f, "out of memory"),
      AllocationError::TooManyBlocks => write!(f, "too many memory blocks allocated"),
    }
  }
}
