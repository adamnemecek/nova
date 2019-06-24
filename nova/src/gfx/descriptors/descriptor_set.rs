// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A set of bound [`Descriptor`]s containing shader resources.
#[derive(Clone)]
pub struct DescriptorSet(Arc<DescriptorSetInner>);

struct DescriptorSetInner {
  pool: DescriptorPool,
  set: Option<backend::DescriptorSet>,
  _descriptors: Vec<Descriptor>,
}

impl DescriptorSet {
  /// Creates a new set from a [`DescriptorPool`] with bindings to the given
  /// `descriptors`.
  ///
  /// The `descriptors` must match those defined in the [`DescriptorLayout`] of
  /// the `pool`.
  pub fn new(
    pool: &DescriptorPool,
    descriptors: impl Into<Vec<Descriptor>>,
  ) -> Result<Self, DescriptorSetCreationError> {
    let context = pool.context();
    let set = pool.acquire()?;
    let descriptors = descriptors.into();

    unsafe {
      context
        .device
        .write_descriptor_sets(descriptors.iter().enumerate().map(|(index, descriptor)| {
          gfx_hal::pso::DescriptorSetWrite {
            set: &set,
            binding: index as u32,
            array_offset: 0,
            descriptors: iter::once(descriptor.as_backend()),
          }
        }));
    }

    Ok(Self(Arc::new(DescriptorSetInner {
      pool: pool.clone(),
      set: Some(set),
      _descriptors: descriptors,
    })))
  }

  /// Returns a reference to the underlying backend descriptor set.
  pub(crate) fn as_backend(&self) -> &backend::DescriptorSet {
    self.0.set.as_ref().unwrap()
  }
}

// Implement `Drop` to release the underlying backend descriptor set back to
// the pool.
impl Drop for DescriptorSetInner {
  fn drop(&mut self) {
    self.pool.release(self.set.take().unwrap());
  }
}

/// An error that occurred during the creation of a new `DescriptorSet`.
#[derive(Debug)]
pub enum DescriptorSetCreationError {
  /// Out of either device or host memory to create resources with.
  OutOfMemory,
  /// Out of memory in the descriptor pool.
  OutOfPoolMemory,
  ///  Memory allocation failed due to pool fragmentation.
  FragmentedPool,
}

impl std::error::Error for DescriptorSetCreationError {}

impl fmt::Display for DescriptorSetCreationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      DescriptorSetCreationError::OutOfMemory => write!(f, "out of memory"),
      DescriptorSetCreationError::OutOfPoolMemory => write!(f, "out of memory in descriptor pool"),
      DescriptorSetCreationError::FragmentedPool => write!(f, "fragmented descriptor pool"),
    }
  }
}

// Implement `From` for allocation errors from the backend.
impl From<gfx_hal::pso::AllocationError> for DescriptorSetCreationError {
  fn from(error: gfx_hal::pso::AllocationError) -> Self {
    match error {
      gfx_hal::pso::AllocationError::OutOfHostMemory => DescriptorSetCreationError::OutOfMemory,
      gfx_hal::pso::AllocationError::OutOfDeviceMemory => DescriptorSetCreationError::OutOfMemory,
      gfx_hal::pso::AllocationError::OutOfPoolMemory => DescriptorSetCreationError::OutOfPoolMemory,
      gfx_hal::pso::AllocationError::FragmentedPool => DescriptorSetCreationError::FragmentedPool,
      gfx_hal::pso::AllocationError::IncompatibleLayout => {
        panic!("failed to allocate descriptor set: incompatible layout")
      }
    }
  }
}
