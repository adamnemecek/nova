pub use gfx_hal::format::Format as TextureFormat;

use super::backend;
use super::prelude::*;
use super::{Buffer, BufferUsage, Device};
use std::iter;
use std::sync::Arc;

pub struct Texture {
  raw: Option<(backend::ImageView, backend::Image)>,
  device: Arc<Device>,
}

impl Texture {
  pub fn new(
    device: &Arc<Device>,
    data: &[u8],
    format: TextureFormat,
    width: u32,
    height: u32,
  ) -> Texture {
    let mut buffer = Buffer::new(device, data.len(), BufferUsage::TRANSFER_SRC);

    buffer.write(data);

    let kind = gfx_hal::image::Kind::D2(width, height, 1, 1);

    let unbound = device
      .raw
      .create_image(
        kind,
        1,
        format,
        gfx_hal::image::Tiling::Optimal,
        gfx_hal::image::Usage::TRANSFER_DST | gfx_hal::image::Usage::SAMPLED,
        gfx_hal::image::ViewCapabilities::empty(),
      )
      .expect("could not create image");

    let requirements = device.raw.get_image_requirements(&unbound);

    let upload_type = device
      .memory_properties
      .memory_types
      .iter()
      .enumerate()
      .find(|(id, ty)| {
        let supported = requirements.type_mask & (1_u64 << id) != 0;

        supported
          && ty
            .properties
            .contains(gfx_hal::memory::Properties::DEVICE_LOCAL)
      })
      .map(|(id, _ty)| gfx_hal::MemoryTypeId(id))
      .expect("could not find approprate vertex buffer memory type");

    let memory = device
      .raw
      .allocate_memory(upload_type, requirements.size)
      .unwrap();

    let image = device
      .raw
      .bind_image_memory(&memory, 0, unbound)
      .expect("could not bind image memory");

    let mut pool = device.one_time_pool.lock().unwrap();
    let pool = pool.as_mut().unwrap();

    let mut cmd_vec = pool.allocate(1, gfx_hal::command::RawLevel::Primary);
    let cmd = &mut cmd_vec[0];

    cmd.begin(
      gfx_hal::command::CommandBufferFlags::ONE_TIME_SUBMIT,
      Default::default(),
    );

    let barrier = gfx_hal::memory::Barrier::Image {
      states: (
        gfx_hal::image::Access::empty(),
        gfx_hal::image::Layout::Undefined,
      )
        ..(
          gfx_hal::image::Access::TRANSFER_WRITE,
          gfx_hal::image::Layout::TransferDstOptimal,
        ),
      target: &image,
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
      &image,
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
      target: &image,
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

    cmd.finish();

    let mut queue = device.command_queue.raw_mut();

    unsafe {
      queue.submit_raw(
        gfx_hal::queue::RawSubmission {
          cmd_buffers: iter::once(cmd),
          wait_semaphores: &[],
          signal_semaphores: &[],
        },
        None,
      );
    }

    queue.wait_idle().expect("wait_idle failed");

    drop(queue);

    unsafe {
      pool.free(cmd_vec);
    }

    drop(pool);

    let view = device
      .raw
      .create_image_view(
        &image,
        gfx_hal::image::ViewKind::D2,
        format,
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

  pub fn raw_view(&self) -> &backend::ImageView {
    &self.raw.as_ref().unwrap().0
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    if let Some((view, image)) = self.raw.take() {
      self.device.raw.destroy_image_view(view);
      self.device.raw.destroy_image(image);
    }
  }
}

pub struct TextureSampler {
  raw: Option<backend::Sampler>,
  device: Arc<Device>,
}

impl TextureSampler {
  pub fn new(device: &Arc<Device>) -> Self {
    let sampler = device.raw.create_sampler(gfx_hal::image::SamplerInfo::new(
      gfx_hal::image::Filter::Linear,
      gfx_hal::image::WrapMode::Tile,
    ));

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
      self.device.raw.destroy_sampler(sampler);
    }
  }
}
