// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod texture;

use self::texture::Texture;
use crate::alloc::Allocator;
use crate::buffer::{Buffer, BufferKind};
use crate::commands::Commands;
use crate::descriptors::{DescriptorKind, DescriptorLayout, DescriptorPool};
use crate::images::{DeviceImageAccess, DeviceImageFormat, DeviceImageLayout};
use crate::pipeline::PipelineStage;
use crate::{Backend, Device, DeviceExt};
use gfx_hal::image::Filter as TextureFilter;
use gfx_hal::image::SamplerInfo as TextureSamplerInfo;
use gfx_hal::image::WrapMode as TextureWrapMode;
use nova_core::collections::{FnvHashMap, FnvHashSet};
use nova_core::ecs;
use nova_core::math::{Rect, Size};
use nova_graphics::images::{self, ImageId};
use nova_graphics::Color4;
use std::mem;

const STAGING_BUFFER_SIZE: usize = 32 * 1024 * 1024;

pub(crate) type TextureSampler = <Backend as gfx_hal::Backend>::Sampler;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureId(u64);

impl TextureId {
  pub const TRANSPARENT: Self = Self(0);
  pub const SOLID: Self = Self(1);
}

pub struct Textures {
  sampler: TextureSampler,
  descriptor_pool: DescriptorPool,
  table: FnvHashMap<TextureId, Texture>,
  next_id: TextureId,
  image_cache: FnvHashMap<ImageId, TextureId>,
  staging_buffer: Buffer,
  staging_offset: usize,
  pending_image_copies: Vec<(TextureId, ImageId)>,
  pending_changes: Vec<(TextureId, Change)>,
  has_pending_changes: FnvHashSet<TextureId>,
}

#[derive(Debug)]
enum Change {
  Clear(Color4),
  CopyStagingBuffer(usize, Rect<u32>),
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

    let staging_buffer = Buffer::new(device, allocator, BufferKind::Staging, STAGING_BUFFER_SIZE);

    let mut textures = Textures {
      sampler,
      descriptor_pool,
      table: FnvHashMap::default(),
      next_id: TextureId(0),
      image_cache: FnvHashMap::default(),
      staging_buffer,
      staging_offset: 0,
      pending_image_copies: Vec::new(),
      pending_changes: Vec::new(),
      has_pending_changes: FnvHashSet::default(),
    };

    textures.create_texture(
      device,
      allocator,
      Size::new(1, 1),
      DeviceImageFormat::Rgba8Unorm,
    );

    textures.create_texture(
      device,
      allocator,
      Size::new(1, 1),
      DeviceImageFormat::Rgba8Unorm,
    );

    textures.clear_texture(TextureId::TRANSPARENT, Color4::TRANSPARENT);
    textures.clear_texture(TextureId::SOLID, Color4::WHITE);

    textures
  }

  pub fn descriptor_layout(&self) -> &DescriptorLayout {
    self.descriptor_pool.layout()
  }

  pub fn transparent(&self) -> &Texture {
    &self.table[&TextureId::TRANSPARENT]
  }

  pub fn solid(&self) -> &Texture {
    &self.table[&TextureId::SOLID]
  }

  pub fn create_texture(
    &mut self,
    device: &Device,
    allocator: &mut Allocator,
    size: Size<u32>,
    format: DeviceImageFormat,
  ) -> TextureId {
    let id = self.next_id;

    self.next_id = TextureId(id.0 + 1);

    self.table.insert(
      id,
      Texture::new(
        device,
        allocator,
        size,
        format,
        &self.sampler,
        &mut self.descriptor_pool,
      ),
    );

    id
  }

  pub fn get_texture(&self, id: TextureId) -> Option<&Texture> {
    self.table.get(&id)
  }

  pub fn clear_texture(&mut self, id: TextureId, color: Color4) {
    self.pending_changes.push((id, Change::Clear(color)));
    self.has_pending_changes.insert(id);
  }

  pub fn copy_to_texture(&mut self, id: TextureId, rect: Rect<u32>, data: &[u8]) {
    let range = self.staging_offset..self.staging_offset + data.len();

    self.staging_buffer[range.clone()].copy_from_slice(data);

    self
      .pending_changes
      .push((id, Change::CopyStagingBuffer(range.start, rect)));

    self.has_pending_changes.insert(id);
    self.staging_offset = range.end - range.end % 4 + 4;
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
    res: &ecs::Resources,
    device: &Device,
    allocator: &mut Allocator,
    cmd: &mut Commands,
  ) {
    let images = images::read(res);
    let mut image_copies = Vec::new();

    mem::swap(&mut self.pending_image_copies, &mut image_copies);

    for (id, image_id) in image_copies.drain(..) {
      let image = match images.get(image_id) {
        Some(image) => image,
        None => continue,
      };

      let size = image.size();

      let sampler = &self.sampler;
      let descriptor_pool = &mut self.descriptor_pool;

      self.table.entry(id).or_insert_with(|| {
        Texture::new(
          device,
          allocator,
          size,
          DeviceImageFormat::Rgba8Unorm,
          sampler,
          descriptor_pool,
        )
      });

      self.copy_to_texture(
        id,
        Rect {
          x1: 0,
          y1: 0,
          x2: size.width,
          y2: size.height,
        },
        image.bytes(),
      )
    }

    if self.pending_changes.is_empty() {
      return;
    }

    // Transition all textures to an optimal transfer layout with a pipeline
    // barrier.
    cmd.pipeline_barrier(
      (PipelineStage::FRAGMENT_SHADER, PipelineStage::TRANSFER),
      self.has_pending_changes.iter().map(|id| {
        self.table[id].image.memory_barrier(
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
    for (id, change) in self.pending_changes.drain(..) {
      let texture = &self.table[&id];

      match change {
        Change::Clear(color) => {
          cmd.clear_image(&texture.image, color);
        }

        Change::CopyStagingBuffer(offset, rect) => {
          cmd.copy_buffer_to_image(&self.staging_buffer, offset, &texture.image, rect);
        }
      }
    }

    // Transition all textures to an optimal layout for shader reads with a
    // pipeline barrier.
    cmd.pipeline_barrier(
      (PipelineStage::TRANSFER, PipelineStage::FRAGMENT_SHADER),
      self.has_pending_changes.iter().map(|id| {
        self.table[id].image.memory_barrier(
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

    self.staging_offset = 0;
    self.has_pending_changes.clear();
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
}
