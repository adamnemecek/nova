// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::TextureSampler;
use crate::alloc::Allocator;
use crate::descriptors::{Descriptor, DescriptorPool, DescriptorSet};
use crate::images::{DeviceImage, DeviceImageFormat};
use crate::Device;
use nova_core::math::Size;

#[derive(Debug)]
pub struct Texture {
  pub image: DeviceImage,
  pub descriptor_set: DescriptorSet,
}

impl Texture {
  pub fn new(
    device: &Device,
    allocator: &mut Allocator,
    size: Size<u32>,
    sampler: &TextureSampler,
    descriptor_pool: &mut DescriptorPool,
  ) -> Texture {
    let image = DeviceImage::new(device, allocator, size, DeviceImageFormat::Rgba8Unorm)
      .expect("Could not create texture image");

    let descriptor_set = descriptor_pool.alloc(device, &[Descriptor::Texture(&image, &sampler)]);

    Texture {
      image,
      descriptor_set,
    }
  }
}
