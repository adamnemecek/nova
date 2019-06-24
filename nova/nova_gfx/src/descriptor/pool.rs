// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A pool of reusable [`DescriptorSet`]s with the same [`DescriptorLayout`].
///
/// This structure is cloneable and all clones refer to the same pool. When all
/// clones and all allocated `DescriptorSet`s have been dropped, the underlying
/// backend resources are destroyed.
#[derive(Clone)]
pub struct DescriptorPool(Arc<DescriptorPoolInner>);

struct DescriptorPoolInner {
  context: Context,
  layout: DescriptorLayout,
  pool: Option<Mutex<backend::DescriptorPool>>,
  recycled_sets: SegQueue<backend::DescriptorSet>,
}

impl DescriptorPool {
  /// Creates a new pool of up to `max_sets` sets with the given `layout`.
  pub fn new(layout: &DescriptorLayout, max_sets: usize) -> Result<Self, OutOfMemoryError> {
    let context = layout.context();

    let ranges = layout
      .kinds()
      .iter()
      .map(|kind| gfx_hal::pso::DescriptorRangeDesc {
        ty: kind.backend_ty(),
        count: max_sets,
      });

    let pool = unsafe {
      context.device.create_descriptor_pool(
        max_sets,
        ranges,
        gfx_hal::pso::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET,
      )?
    };

    Ok(Self(Arc::new(DescriptorPoolInner {
      context: context.clone(),
      layout: layout.clone(),
      pool: Some(Mutex::new(pool)),
      recycled_sets: SegQueue::new(),
    })))
  }

  /// Returns a reference to the graphics context the pool was created in.
  pub fn context(&self) -> &Context {
    &self.0.context
  }

  /// Acquires a backend descriptor set from the pool.
  pub(crate) fn acquire(&self) -> Result<backend::DescriptorSet, gfx_hal::pso::AllocationError> {
    // First try to recycle a previously allocated set that was dropped.
    if let Ok(set) = self.0.recycled_sets.pop() {
      return Ok(set);
    }

    // Otherwise allocate a new one.
    let mut pool = self.0.pool.as_ref().unwrap().lock();

    unsafe { pool.allocate_set(&self.0.layout.as_backend()) }
  }

  /// Releases a backend descriptor set back to the pool for reuse.
  pub(crate) fn release(&self, set: backend::DescriptorSet) {
    self.0.recycled_sets.push(set);
  }
}

// Implement `Drop` to free the underlying resources.
impl Drop for DescriptorPoolInner {
  fn drop(&mut self) {
    let mut pool = self.pool.take().unwrap().into_inner();

    unsafe {
      while let Ok(set) = self.recycled_sets.pop() {
        pool.free_sets(iter::once(set));
      }

      self.context.device.destroy_descriptor_pool(pool);
    }
  }
}
