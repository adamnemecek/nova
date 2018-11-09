pub use gfx_hal::format::Format as ImageFormat;

use super::{CommandBuffer, CommandBufferKind, CommandPool};
use crate::graphics::device::{self, Device};
use crate::graphics::hal::*;
use gfx_memory::Factory;
use std::borrow::Borrow;
use std::iter;
use std::sync::Arc;

pub type Allocation = <device::Allocator as Factory<Backend>>::Image;

pub struct Texture {
  raw: Option<(backend::ImageView, Allocation)>,
  device: Arc<Device>,
}

impl Texture {
  pub fn raw_view(&self) -> &backend::ImageView {
    &self.raw.as_ref().unwrap().0
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    if let Some((view, image)) = self.raw.take() {
      self.device.raw().destroy_image_view(view);

      self
        .device
        .allocator()
        .destroy_image(self.device.raw(), image);
    }
  }
}

pub struct TextureSampler {
  raw: Option<backend::Sampler>,
  device: Arc<Device>,
}

impl TextureSampler {
  pub fn new(device: &Arc<Device>) -> Self {
    let sampler = device
      .raw()
      .create_sampler(gfx_hal::image::SamplerInfo::new(
        gfx_hal::image::Filter::Linear,
        gfx_hal::image::WrapMode::Tile,
      ))
      .expect("could not create sampler");

    TextureSampler {
      device: device.clone(),
      raw: Some(sampler),
    }
  }

  pub fn raw(&self) -> &backend::Sampler {
    self.raw.as_ref().unwrap()
  }
}

impl Drop for TextureSampler {
  fn drop(&mut self) {
    if let Some(sampler) = self.raw.take() {
      self.device.raw().destroy_sampler(sampler);
    }
  }
}

pub struct TextureLoader {
  command_pool: Arc<CommandPool>,
}

impl TextureLoader {
  pub fn new(queue: &Arc<device::Queue>) -> TextureLoader {
    TextureLoader {
      command_pool: CommandPool::new(queue),
    }
  }

  pub fn load(&mut self, source: &image::RgbaImage) -> Texture {
    let mut cmd = CommandBuffer::new(&self.command_pool, CommandBufferKind::Primary);
    let device = self.command_pool.queue().device();

    let (width, height) = source.dimensions();

    let mut buffer = device::Buffer::new(device, source.len(), device::BufferUsage::TRANSFER_SRC);

    buffer.write(&source);

    let image = device
      .allocator()
      .create_image(
        device.raw(),
        (
          gfx_memory::Type::General,
          gfx_hal::memory::Properties::DEVICE_LOCAL,
        ),
        gfx_hal::image::Kind::D2(width, height, 1, 1),
        1,
        gfx_hal::format::Format::Rgba8Srgb,
        gfx_hal::image::Tiling::Optimal,
        gfx_hal::image::Usage::TRANSFER_DST | gfx_hal::image::Usage::SAMPLED,
        gfx_hal::image::ViewCapabilities::empty(),
      )
      .expect("could not create image");

    cmd.begin();

    cmd.record_raw(|cmd| {
      let barrier = gfx_hal::memory::Barrier::Image {
        states: (
          gfx_hal::image::Access::empty(),
          gfx_hal::image::Layout::Undefined,
        )
          ..(
            gfx_hal::image::Access::TRANSFER_WRITE,
            gfx_hal::image::Layout::TransferDstOptimal,
          ),
        target: image.borrow(),
        range: gfx_hal::image::SubresourceRange {
          aspects: gfx_hal::format::Aspects::COLOR,
          levels: 0..1,
          layers: 0..1,
        },
      };

      cmd.pipeline_barrier(
        gfx_hal::pso::PipelineStage::TOP_OF_PIPE..gfx_hal::pso::PipelineStage::TRANSFER,
        gfx_hal::memory::Dependencies::empty(),
        &[barrier],
      );

      cmd.copy_buffer_to_image(
        buffer.raw(),
        image.borrow(),
        gfx_hal::image::Layout::TransferDstOptimal,
        &[gfx_hal::command::BufferImageCopy {
          buffer_offset: 0,
          buffer_width: 0,
          buffer_height: 0,
          image_layers: gfx_hal::image::SubresourceLayers {
            aspects: gfx_hal::format::Aspects::COLOR,
            level: 0,
            layers: 0..1,
          },
          image_offset: gfx_hal::image::Offset { x: 0, y: 0, z: 0 },
          image_extent: gfx_hal::image::Extent {
            width,
            height,
            depth: 1,
          },
        }],
      );

      let barrier = gfx_hal::memory::Barrier::Image {
        states: (
          gfx_hal::image::Access::TRANSFER_WRITE,
          gfx_hal::image::Layout::TransferDstOptimal,
        )
          ..(
            gfx_hal::image::Access::SHADER_READ,
            gfx_hal::image::Layout::ShaderReadOnlyOptimal,
          ),
        target: image.borrow(),
        range: gfx_hal::image::SubresourceRange {
          aspects: gfx_hal::format::Aspects::COLOR,
          levels: 0..1,
          layers: 0..1,
        },
      };

      cmd.pipeline_barrier(
        gfx_hal::pso::PipelineStage::TRANSFER..gfx_hal::pso::PipelineStage::FRAGMENT_SHADER,
        gfx_hal::memory::Dependencies::empty(),
        &[barrier],
      );
    });

    cmd.finish();

    let mut queue = self.command_pool.queue().raw_mut();

    unsafe {
      queue.submit_raw(
        gfx_hal::queue::RawSubmission {
          cmd_buffers: iter::once(cmd.raw()),
          wait_semaphores: &[],
          signal_semaphores: &[],
        },
        None,
      );
    }

    queue.wait_idle().expect("wait_idle failed");

    drop(cmd);

    let view = device
      .raw()
      .create_image_view(
        image.borrow(),
        gfx_hal::image::ViewKind::D2,
        gfx_hal::format::Format::Rgba8Srgb,
        gfx_hal::format::Swizzle::NO,
        gfx_hal::image::SubresourceRange {
          aspects: gfx_hal::format::Aspects::COLOR,
          levels: 0..1,
          layers: 0..1,
        },
      )
      .expect("could not create image view");

    Texture {
      device: device.clone(),
      raw: Some((view, image)),
    }
  }
}
