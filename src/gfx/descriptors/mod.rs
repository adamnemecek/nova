// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod descriptor;
mod descriptor_layout;
mod descriptor_pool;
mod descriptor_set;

pub use self::descriptor::*;
pub use self::descriptor_layout::*;
pub use self::descriptor_pool::*;
pub use self::descriptor_set::*;

use super::*;

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
