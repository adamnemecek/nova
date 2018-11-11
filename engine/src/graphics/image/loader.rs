use super::{Backing, Format, Image, Source};
use crate::graphics::buffer::{self, Buffer};
use crate::graphics::commands::{self, CommandPool, Commands};
use crate::graphics::device;
use crate::graphics::hal::prelude::*;
use gfx_memory::Factory;
use std::borrow::Borrow;
use std::iter;
use std::sync::Arc;

pub struct Loader {
  command_pool: Arc<CommandPool>,
}

impl Loader {
  pub fn new(queue: &Arc<device::Queue>) -> Self {
    Loader {
      command_pool: CommandPool::new(queue),
    }
  }

  pub fn load(&mut self, source: &Source) -> Image {
    let mut cmd = Commands::new(&self.command_pool, commands::Level::Primary);
    let device = self.command_pool.queue().device();

    let size = source.size();

    let mut buffer = Buffer::new(device, source.bytes().len(), buffer::Usage::TRANSFER_SRC);

    buffer.write(source.bytes());

    let image = device
      .allocator()
      .create_image(
        device.raw(),
        (
          gfx_memory::Type::General,
          gfx_hal::memory::Properties::DEVICE_LOCAL,
        ),
        gfx_hal::image::Kind::D2(size.x, size.y, 1, 1),
        1,
        Format::Rgba8Srgb,
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
            width: size.x,
            height: size.y,
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
          cmd_buffers: iter::once(cmd.as_ref()),
          wait_semaphores: &[],
          signal_semaphores: &[],
        },
        None,
      );
    }

    queue.wait_idle().expect("wait_idle failed");

    drop(cmd);

    Image::from_raw(device, Backing::Allocated(image), Format::Rgba8Srgb, size)
  }
}
