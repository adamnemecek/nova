// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Descriptor, DescriptorLayout};
use crate::graphics::prelude::*;
use crate::utils::Droppable;
use crossbeam::queue::MsQueue;
use std::iter;
use std::sync::{Arc, Weak};

/// A set of descriptors which can be bound to a command buffer and used in a
/// graphics pipeline to reference images, buffers, and other data from shaders.
pub struct DescriptorSet {
  /// Raw backend descriptor set structure.
  raw: Droppable<backend::DescriptorSet>,
  /// Weak reference to a recycling queue. When this set is dropped, the raw
  /// backend set structure is added to this queue for reuse.
  recycling: Weak<MsQueue<backend::DescriptorSet>>,
  /// List of descriptors in the set.
  descriptors: Vec<Descriptor>,
}

impl DescriptorSet {
  /// Gets a list of the descriptors in the set.
  pub fn descriptors(&self) -> &[Descriptor] {
    &self.descriptors
  }
}

// Implement `AsRef` to expose the raw backend descriptor set.
impl AsRef<backend::DescriptorSet> for DescriptorSet {
  fn as_ref(&self) -> &backend::DescriptorSet {
    &self.raw
  }
}

// Implement `Drop` to return the raw backend descriptor set to the recycling
// queue for the pool.
impl Drop for DescriptorSet {
  fn drop(&mut self) {
    if let Some(raw) = self.raw.take() {
      if let Some(recycling) = self.recycling.upgrade() {
        recycling.push(raw);
      }
    }
  }
}

/// A pool of descriptor sets for a specific descriptor layout.
pub struct DescriptorPool {
  /// Raw backend descriptor pool structure.
  raw: Droppable<backend::DescriptorPool>,
  /// Layout of the descriptor sets in the pool.
  layout: Arc<DescriptorLayout>,
  /// Atomic queue of previously allocated raw descriptor sets to reuse.
  recycling: Arc<MsQueue<backend::DescriptorSet>>,
}

impl DescriptorPool {
  /// Creates a new descriptor pool with a maximum number of descriptor sets for
  /// the given layout.
  pub fn new(layout: &Arc<DescriptorLayout>, max_sets: usize) -> Self {
    let ranges = layout
      .raw_bindings()
      .iter()
      .map(|binding| hal::pso::DescriptorRangeDesc {
        ty: binding.ty,
        count: binding.count,
      });

    let pool = layout
      .device()
      .raw()
      .create_descriptor_pool(max_sets, ranges)
      .expect("Could not create backend descriptor pool");

    DescriptorPool {
      layout: layout.clone(),
      raw: pool.into(),
      recycling: Arc::new(MsQueue::new()),
    }
  }
}

impl DescriptorPool {
  /// Allocates a single descriptor set from the pool and writes the given
  /// descriptors to it.
  pub fn allocate_set(&mut self, descriptors: Vec<Descriptor>) -> DescriptorSet {
    // TODO: Debug assertion to check that descriptors match layout.

    let set = self
      .raw
      .allocate_set(self.layout.as_ref().as_ref())
      .expect("Could not allocate backend descriptor set");

    self
      .layout
      .device()
      .raw()
      .write_descriptor_sets(iter::once(hal::pso::DescriptorSetWrite {
        set: &set,
        binding: 0,
        array_offset: 0,
        descriptors: descriptors.iter().map(hal::pso::Descriptor::from),
      }));

    DescriptorSet {
      raw: set.into(),
      recycling: Arc::downgrade(&self.recycling),
      descriptors,
    }
  }
}

// Implement `Drop` to destroy the raw backend descriptor pool.
impl Drop for DescriptorPool {
  fn drop(&mut self) {
    if let Some(pool) = self.raw.take() {
      self.layout.device().raw().destroy_descriptor_pool(pool);
    }
  }
}
