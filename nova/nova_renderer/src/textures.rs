// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod texture;

use self::texture::Texture;
use crate::alloc::Allocator;
use crate::buffer::{Buffer, BufferKind};
use crate::commands::Commands;
use crate::descriptors::{DescriptorKind, DescriptorLayout, DescriptorPool};
use crate::images::{DeviceImageAccess, DeviceImageLayout};
use crate::pipeline::PipelineStage;
use crate::{Backend, Device, DeviceExt};
use gfx_hal::image::Filter as TextureFilter;
use gfx_hal::image::SamplerInfo as TextureSamplerInfo;
use gfx_hal::image::WrapMode as TextureWrapMode;
use nova_core::engine::Resources;
use nova_core::math::Size;
use nova_graphics::images::{self, ImageId};
use nova_graphics::Color4;
use std::collections::HashMap;
use std::ops::Range;

pub(crate) type TextureSampler = <Backend as gfx_hal::Backend>::Sampler;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureId(u64);

pub struct Textures {
  sampler: TextureSampler,
  descriptor_pool: DescriptorPool,
  table: HashMap<TextureId, Texture>,
  next_id: TextureId,
  image_cache: HashMap<ImageId, TextureId>,
  staging_buffer: Buffer,
  staging_offset: usize,
  pending_image_copies: Vec<(TextureId, ImageId)>,
  pending_changes: Vec<(TextureId, Change)>,
}

#[derive(Debug)]
enum Change {
  Clear(Color4),
  CopyStagingBuffer(Range<usize>),
}

impl Textures {
  pub fn new(device: &Device, allocator: &mut Allocator) -> Self {
    let sampler = unsafe {
      device
        .create_sampler(TextureSamplerInfo::new(
          TextureFilter::Nearest,
          TextureWrapMode::Tile,
        ))
        .expect("Could not create texture sampler")
    };

    let descriptor_pool = DescriptorPool::new(
      device,
      DescriptorLayout::new(device, vec![DescriptorKind::SampledTexture]),
    );

    let staging_buffer = Buffer::new(device, allocator, BufferKind::Staging, 128 * 1024 * 1024);

    let mut textures = Textures {
      sampler,
      descriptor_pool,
      table: HashMap::new(),
      next_id: TextureId(2),
      image_cache: HashMap::new(),
      staging_buffer,
      staging_offset: 0,
      pending_image_copies: Vec::new(),
      pending_changes: vec![
        (TextureId(0), Change::Clear(Color4::TRANSPARENT)),
        (TextureId(1), Change::Clear(Color4::WHITE)),
      ],
    };

    textures.insert_new(device, allocator, TextureId(0), Size::new(1, 1));
    textures.insert_new(device, allocator, TextureId(1), Size::new(1, 1));

    textures
  }

  pub fn descriptor_layout(&self) -> &DescriptorLayout {
    self.descriptor_pool.layout()
  }

  pub fn transparent_id(&self) -> TextureId {
    TextureId(0)
  }

  pub fn transparent(&self) -> &Texture {
    &self.table[&self.transparent_id()]
  }

  pub fn solid_id(&self) -> TextureId {
    TextureId(1)
  }

  pub fn solid(&self) -> &Texture {
    &self.table[&self.solid_id()]
  }

  pub fn get(&self, id: TextureId) -> Option<&Texture> {
    self.table.get(&id)
  }

  pub fn get_or_transparent(&self, id: TextureId) -> &Texture {
    self.table.get(&id).unwrap_or_else(|| self.transparent())
  }

  pub fn get_or_solid(&self, id: TextureId) -> &Texture {
    self.table.get(&id).unwrap_or_else(|| self.solid())
  }

  pub fn cache_image(&mut self, image_id: ImageId) -> TextureId {
    match self.image_cache.get(&image_id) {
      Some(id) => *id,

      None => {
        let id = self.next_id;

        self.next_id = TextureId(self.next_id.0 + 1);

        self.image_cache.insert(image_id, id);
        self.pending_image_copies.push((id, image_id));

        id
      }
    }
  }

  pub(crate) fn flush_changes(
    &mut self,
    res: &Resources,
    device: &Device,
    allocator: &mut Allocator,
    cmd: &mut Commands,
  ) {
    let images = images::read(res);
    let sampler = &self.sampler;
    let descriptor_pool = &mut self.descriptor_pool;

    for (id, image_id) in self.pending_image_copies.drain(..) {
      let image = match images.get(image_id) {
        Some(image) => image,
        None => continue,
      };

      self
        .table
        .entry(id)
        .or_insert_with(|| Texture::new(device, allocator, image.size(), sampler, descriptor_pool));

      let bytes = image.bytes();
      let range = self.staging_offset..self.staging_offset + bytes.len();

      self.staging_offset = range.end;
      self.staging_buffer[range.clone()].copy_from_slice(bytes);

      self
        .pending_changes
        .push((id, Change::CopyStagingBuffer(range)));
    }

    if self.pending_changes.is_empty() {
      return;
    }

    // Transition all textures to an optimal transfer layout with a pipeline
    // barrier.
    cmd.pipeline_barrier(
      (PipelineStage::FRAGMENT_SHADER, PipelineStage::TRANSFER),
      self.pending_changes.iter().map(|(id, _)| {
        self.get(*id).unwrap().image.memory_barrier(
          (
            DeviceImageAccess::empty(),
            DeviceImageAccess::TRANSFER_WRITE,
          ),
          (
            DeviceImageLayout::Undefined,
            DeviceImageLayout::TransferDstOptimal,
          ),
        )
      }),
    );

    // Record commands for each pending change.
    for (id, change) in &self.pending_changes {
      let texture = self.get(*id).unwrap();

      match change {
        Change::Clear(color) => {
          cmd.clear_image(&texture.image, *color);
        }

        Change::CopyStagingBuffer(range) => {
          cmd.copy_buffer_to_image(&self.staging_buffer, range.start, &texture.image);
        }
      }
    }

    // Transition all textures to an optimal layout for shader reads with a
    // pipeline barrier.
    cmd.pipeline_barrier(
      (PipelineStage::TRANSFER, PipelineStage::FRAGMENT_SHADER),
      self.pending_changes.iter().map(|(id, _)| {
        self.get(*id).unwrap().image.memory_barrier(
          (
            DeviceImageAccess::TRANSFER_WRITE,
            DeviceImageAccess::SHADER_READ,
          ),
          (
            DeviceImageLayout::TransferDstOptimal,
            DeviceImageLayout::ShaderReadOnlyOptimal,
          ),
        )
      }),
    );

    self.pending_changes.clear();
  }

  pub fn destroy(self, device: &Device, allocator: &mut Allocator) {
    for (_, texture) in self.table.into_iter() {
      texture.image.destroy(device, allocator);
    }

    self.descriptor_pool.destroy(device);
    self.staging_buffer.destroy(device, allocator);

    unsafe {
      device.destroy_sampler(self.sampler);
    }
  }

  fn insert_new(
    &mut self,
    device: &Device,
    allocator: &mut Allocator,
    id: TextureId,
    size: Size<u32>,
  ) -> Option<Texture> {
    self.table.insert(
      id,
      Texture::new(
        device,
        allocator,
        size,
        &self.sampler,
        &mut self.descriptor_pool,
      ),
    )
  }
}
