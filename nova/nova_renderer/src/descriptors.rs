// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod layout;
mod pool;

pub use self::layout::DescriptorLayout;
pub use self::pool::DescriptorPool;

use crate::images::{DeviceImage, DeviceImageLayout};
use crate::textures::TextureSampler;
use crate::Backend;

pub type DescriptorSet = <Backend as gfx_hal::Backend>::DescriptorSet;

#[derive(Debug, Clone, Copy)]
pub enum DescriptorKind {
  SampledTexture,
}

impl DescriptorKind {
  fn count(self) -> usize {
    1
  }

  fn stage_flags(self) -> gfx_hal::pso::ShaderStageFlags {
    gfx_hal::pso::ShaderStageFlags::FRAGMENT
  }

  fn ty(self) -> gfx_hal::pso::DescriptorType {
    match self {
      DescriptorKind::SampledTexture => gfx_hal::pso::DescriptorType::CombinedImageSampler,
    }
  }
}

#[derive(Debug)]
pub enum Descriptor<'a> {
  Texture(&'a DeviceImage, &'a TextureSampler),
}

impl<'a> From<&Descriptor<'a>> for gfx_hal::pso::Descriptor<'a, Backend> {
  fn from(desc: &Descriptor<'a>) -> Self {
    match *desc {
      Descriptor::Texture(image, sampler) => gfx_hal::pso::Descriptor::CombinedImageSampler(
        &image.raw_view,
        DeviceImageLayout::ShaderReadOnlyOptimal,
        sampler,
      ),
    }
  }
}
