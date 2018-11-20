// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::device::DeviceHandle;
use crate::graphics::prelude::*;
use crate::utils::Droppable;

/// An object that describes the layout of a descriptor set.
///
/// Descriptor layouts are a list of numbered bindings of different types of
/// data.
pub struct DescriptorLayout {
  /// Device the descriptor layout was created with.
  device: DeviceHandle,
  /// Raw backend descriptor layout struct.
  raw: Droppable<backend::DescriptorSetLayout>,
  /// Raw list of bindings in the descriptor layout.
  bindings: Vec<hal::pso::DescriptorSetLayoutBinding>,
}

impl DescriptorLayout {
  /// Creates a new descriptor layout on the device with the given bindings.
  pub fn new(device: &DeviceHandle, bindings: &[DescriptorBinding]) -> Self {
    let bindings = bindings
      .iter()
      .enumerate()
      .map(|(i, binding)| binding.into_hal(i))
      .collect();

    let layout = device
      .raw()
      .create_descriptor_set_layout(&bindings, &[])
      .expect("Could not create backend descriptor set layout");

    DescriptorLayout {
      device: device.clone(),
      raw: layout.into(),
      bindings,
    }
  }

  /// Gets the device the descriptor layout was created with.
  pub fn device(&self) -> &DeviceHandle {
    &self.device
  }

  /// Gets the raw list of bindings in the descriptor layout.
  pub fn raw_bindings(&self) -> &[hal::pso::DescriptorSetLayoutBinding] {
    &self.bindings
  }
}

// Implement `AsRef` to expose the raw backend descriptor layout.
impl AsRef<backend::DescriptorSetLayout> for DescriptorLayout {
  fn as_ref(&self) -> &backend::DescriptorSetLayout {
    &self.raw
  }
}

// Implement `Drop` to destroy the raw backend descriptor layout.
impl Drop for DescriptorLayout {
  fn drop(&mut self) {
    if let Some(layout) = self.raw.take() {
      self.device.raw().destroy_descriptor_set_layout(layout);
    }
  }
}

/// One of the possible bindings in a descriptor layout.
#[derive(Clone, Copy)]
pub enum DescriptorBinding {
  /// A combination of an [`Image`] and a [`Sampler`].
  Texture,
}

impl DescriptorBinding {
  /// Converts this binding into the equivalent gfx-hal structure using the
  /// given binding index.
  fn into_hal(self, index: usize) -> hal::pso::DescriptorSetLayoutBinding {
    hal::pso::DescriptorSetLayoutBinding {
      binding: index as u32,
      count: 1,
      immutable_samplers: false,
      ty: match self {
        DescriptorBinding::Texture => hal::pso::DescriptorType::CombinedImageSampler,
      },
      stage_flags: match self {
        DescriptorBinding::Texture => {
          hal::pso::ShaderStageFlags::FRAGMENT | hal::pso::ShaderStageFlags::VERTEX
        }
      },
    }
  }
}
