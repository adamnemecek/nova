// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use gfx_hal::DescriptorPool as _;
use nova_sync::queue::SegQueue;

#[derive(Debug, Clone, Copy)]
pub enum DescriptorKind {
  UniformBuffer,
  SampledImage,
}

impl DescriptorKind {
  fn backend_ty(self) -> gfx_hal::pso::DescriptorType {
    match self {
      DescriptorKind::UniformBuffer => gfx_hal::pso::DescriptorType::UniformBuffer,
      DescriptorKind::SampledImage => gfx_hal::pso::DescriptorType::CombinedImageSampler,
    }
  }
}

#[derive(Clone)]
pub struct DescriptorLayout(Arc<DescriptorLayoutInner>);

struct DescriptorLayoutInner {
  context: Context,
  layout: Option<backend::DescriptorLayout>,
  kinds: Vec<DescriptorKind>,
}

impl DescriptorLayout {
  pub fn new(
    context: &Context,
    kinds: impl Into<Vec<DescriptorKind>>,
  ) -> Result<Self, OutOfMemoryError> {
    let kinds = kinds.into();

    let bindings =
      kinds
        .iter()
        .enumerate()
        .map(|(index, kind)| gfx_hal::pso::DescriptorSetLayoutBinding {
          binding: index as u32,
          ty: kind.backend_ty(),
          count: 1,
          stage_flags: gfx_hal::pso::ShaderStageFlags::ALL,
          immutable_samplers: false,
        });

    let layout = unsafe { context.device.create_descriptor_set_layout(bindings, &[])? };

    Ok(Self(Arc::new(DescriptorLayoutInner {
      context: context.clone(),
      layout: Some(layout),
      kinds,
    })))
  }

  pub(crate) fn as_backend(&self) -> &backend::DescriptorLayout {
    self.0.layout.as_ref().unwrap()
  }
}

impl Drop for DescriptorLayoutInner {
  fn drop(&mut self) {
    unsafe {
      self
        .context
        .device
        .destroy_descriptor_set_layout(self.layout.take().unwrap());
    }
  }
}

#[derive(Clone)]
pub struct DescriptorPool(Arc<DescriptorPoolInner>);

struct DescriptorPoolInner {
  context: Context,
  layout: DescriptorLayout,
  pool: Option<Mutex<backend::DescriptorPool>>,
  recycled_sets: SegQueue<backend::DescriptorSet>,
}

impl DescriptorPool {
  pub fn new(layout: &DescriptorLayout, max_sets: usize) -> Result<Self, OutOfMemoryError> {
    let context = &layout.0.context;

    let ranges = layout
      .0
      .kinds
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

  pub(crate) fn alloc(&self) -> Result<backend::DescriptorSet, gfx_hal::pso::AllocationError> {
    if let Ok(set) = self.0.recycled_sets.pop() {
      return Ok(set);
    }

    let mut pool = self.0.pool.as_ref().unwrap().lock();

    unsafe { pool.allocate_set(&self.0.layout.as_backend()) }
  }
}

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

#[derive(Clone)]
pub struct DescriptorSet(Arc<DescriptorSetInner>);

struct DescriptorSetInner {
  pool: DescriptorPool,
  set: Option<backend::DescriptorSet>,
  _descriptors: Vec<Descriptor>,
}

impl DescriptorSet {
  pub fn new(
    pool: &DescriptorPool,
    descriptors: impl Into<Vec<Descriptor>>,
  ) -> Result<Self, DescriptorSetCreationError> {
    let context = &pool.0.context;
    let set = pool.alloc()?;
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

  pub(crate) fn as_backend(&self) -> &backend::DescriptorSet {
    self.0.set.as_ref().unwrap()
  }
}

impl Drop for DescriptorSetInner {
  fn drop(&mut self) {
    self.pool.0.recycled_sets.push(self.set.take().unwrap());
  }
}

pub enum Descriptor {
  UniformBuffer(Buffer),
  SampledImage(Image, Sampler),
}

impl Descriptor {
  pub(crate) fn as_backend(&self) -> backend::Descriptor {
    match self {
      Descriptor::UniformBuffer(buffer) => {
        gfx_hal::pso::Descriptor::Buffer(buffer.as_backend(), Some(0)..Some(buffer.len()))
      }
      Descriptor::SampledImage(image, sampler) => gfx_hal::pso::Descriptor::CombinedImageSampler(
        image.as_backend_view(),
        gfx_hal::image::Layout::ShaderReadOnlyOptimal,
        sampler.as_backend(),
      ),
    }
  }
}

// An error that occurred during the creation of a new `DescriptorSet`.
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

impl fmt::Display for DescriptorSetCreationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      DescriptorSetCreationError::OutOfMemory => write!(f, "out of memory"),
      DescriptorSetCreationError::OutOfPoolMemory => write!(f, "out of memory in descriptor pool"),
      DescriptorSetCreationError::FragmentedPool => write!(f, "fragmented descriptor pool"),
    }
  }
}
