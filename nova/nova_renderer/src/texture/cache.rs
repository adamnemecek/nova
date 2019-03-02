// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Sampler, Texture, TextureAccess, TextureLayout};
use crate::buffer::{Buffer, BufferKind};
use crate::descriptors::{
  Descriptor, DescriptorKind, DescriptorLayout, DescriptorPool, DescriptorSet,
};
use crate::pipeline::PipelineStage;
use crate::{Allocator, Commands, Device, DeviceExt};
use nova_graphics as graphics;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct TextureCache {
  table: BTreeMap<usize, TextureId>,
  entries: Vec<Option<Entry>>,
  descriptor_pool: DescriptorPool,
  changes: Vec<(usize, graphics::Image)>,
  staging_buffer: Buffer,
  sampler: Sampler,
}

#[derive(Debug)]
struct Entry {
  texture: Texture,
  descriptor_set: DescriptorSet,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextureId {
  index: usize,
  image: usize,
}

impl TextureCache {
  pub fn new(device: &Device, allocator: &mut Allocator) -> Self {
    let descriptor_pool = DescriptorPool::new(
      device,
      DescriptorLayout::new(device, vec![DescriptorKind::SampledTexture]),
    );

    let staging_buffer = Buffer::new(device, allocator, BufferKind::Staging, 1024 * 1024 * 128);

    let sampler = unsafe {
      device
        .create_sampler(gfx_hal::image::SamplerInfo::new(
          gfx_hal::image::Filter::Linear,
          gfx_hal::image::WrapMode::Tile,
        ))
        .expect("Could not create texture sampler")
    };

    let mut cache = TextureCache {
      table: BTreeMap::new(),
      entries: Vec::new(),
      descriptor_pool,
      changes: Vec::new(),
      staging_buffer,
      sampler,
    };

    // Load the default texture into the cache.
    cache.get_cached(
      &graphics::Image::from_bytes(include_bytes!("1x1.png")).unwrap(),
      device,
      allocator,
    );

    cache
  }

  pub fn descriptor_layout(&self) -> &DescriptorLayout {
    &self.descriptor_pool.layout
  }

  pub fn get_default(&self) -> &DescriptorSet {
    &self.entries[0].as_ref().unwrap().descriptor_set
  }

  pub fn get_cached(
    &mut self,
    image: &graphics::Image,
    device: &Device,
    allocator: &mut Allocator,
  ) -> &DescriptorSet {
    let image_addr = &image.bytes()[0] as *const _ as usize;

    let index = match self.table.get(&image_addr) {
      Some(id) => id.index,
      None => {
        let id = TextureId {
          index: self.entries.len(),
          image: image_addr,
        };

        self.table.insert(image_addr, id);
        self.entries.push(None);

        id.index
      }
    };

    let entry = &mut self.entries[index];

    match entry {
      Some(entry) => &entry.descriptor_set,

      None => {
        let texture = Texture::new(device, allocator, image.size());

        let descriptor_set = self.descriptor_pool.alloc(
          device,
          &[Descriptor::SampledTexture(&texture, &self.sampler)],
        );

        self.changes.push((index, image.clone()));

        *entry = Some(Entry {
          texture,
          descriptor_set,
        });

        &entry.as_ref().unwrap().descriptor_set
      }
    }
  }

  pub fn record_commands(&mut self, cmd: &mut Commands) {
    if self.changes.is_empty() {
      return;
    }

    // Transition all textures to an optimal transfer layout with a pipeline
    // barrier.
    cmd.pipeline_barrier(
      (PipelineStage::FRAGMENT_SHADER, PipelineStage::TRANSFER),
      self.changes.iter().map(|(index, _)| {
        self.get_texture(*index).barrier(
          (TextureAccess::empty(), TextureAccess::TRANSFER_WRITE),
          (TextureLayout::Undefined, TextureLayout::TransferDstOptimal),
        )
      }),
    );

    // Copy all images into the staging buffer and record a copy command for
    // each.
    let mut offset = 0;

    for (index, image) in &self.changes {
      let bytes = image.bytes();
      let size = bytes.len();

      self.staging_buffer[offset..offset + size].copy_from_slice(bytes);

      let texture = self.get_texture(*index);

      cmd.copy_buffer_to_texture(&self.staging_buffer, offset, texture);

      offset += size;
    }

    // Transition all textures to an optimal layout for shader reads with a
    // pipeline barrier.
    cmd.pipeline_barrier(
      (PipelineStage::TRANSFER, PipelineStage::FRAGMENT_SHADER),
      self.changes.iter().map(|(index, _)| {
        self.get_texture(*index).barrier(
          (TextureAccess::TRANSFER_WRITE, TextureAccess::SHADER_READ),
          (
            TextureLayout::TransferDstOptimal,
            TextureLayout::ShaderReadOnlyOptimal,
          ),
        )
      }),
    );

    self.changes.clear();
  }

  fn get_texture(&self, index: usize) -> &Texture {
    &self.entries[index].as_ref().unwrap().texture
  }

  pub fn destroy(self, device: &Device, allocator: &mut Allocator) {
    for entry in self.entries {
      if let Some(entry) = entry {
        entry.texture.destroy(device, allocator);
      }
    }

    self.descriptor_pool.destroy(device);
    self.staging_buffer.destroy(device, allocator);

    unsafe {
      device.destroy_sampler(self.sampler);
    }
  }
}
