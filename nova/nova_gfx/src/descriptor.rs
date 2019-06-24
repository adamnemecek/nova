// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod layout;
mod pool;
mod set;

pub use self::layout::*;
pub use self::pool::*;
pub use self::set::*;

use super::*;
use gfx_hal::DescriptorPool as _;
use nova_sync::queue::SegQueue;

/// One of the possible kinds of resource descriptor.
#[derive(Debug, Clone, Copy)]
pub enum DescriptorKind {
  /// A `Buffer` with kind `BufferKind::Uniform`, accessible to shaders as a
  /// `uniform` resource.
  UniformBuffer,
  /// A combined `Image` and `Sampler`, accessible to shaders as a `sampler1D`,
  /// `sampler2D`, or `sampler3D` resource.
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

/// Represents a device resource descriptor containing resources to use in
/// shaders.
pub enum Descriptor {
  /// A `Buffer` with kind `BufferKind::Uniform`, accessible to shaders as a
  /// `uniform` resource.
  UniformBuffer(Buffer),
  /// A combined `Image` and `Sampler`, accessible to shaders as a `sampler1D`,
  /// `sampler2D`, or `sampler3D` resource.
  SampledImage(Image, Sampler),
}

impl Descriptor {
  /// Returns a backend descriptor definition referencing the contained
  /// resources.
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
