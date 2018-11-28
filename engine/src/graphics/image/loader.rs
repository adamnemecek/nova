// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Backing, Format, Image, Source};
use crate::graphics::buffer::{Buffer, BufferUsage};
use crate::graphics::commands::{CommandLevel, CommandPool, Commands};
use crate::graphics::device;
use crate::graphics::prelude::*;
use gfx_memory::Factory;
use std::borrow::Borrow;
use std::iter;
use std::sync::Arc;

pub struct Loader {
  device: device::Handle,
  command_pool: Arc<CommandPool>,
}

impl Loader {
  pub fn new(device: &device::Handle, queue_family_id: usize) -> Self {
    Loader {
      device: device.clone(),
      command_pool: Arc::new(CommandPool::new(device, queue_family_id)),
    }
  }

  pub fn load(&mut self, queue: &mut device::Queue, source: &Source) -> Image {
    let device = &self.device;
    let mut cmd = Commands::new(&self.command_pool, CommandLevel::Primary);

    let size = source.size();

    let mut buffer = Buffer::new(device, source.bytes().len(), BufferUsage::TRANSFER_SRC);

    buffer.write(source.bytes());

    let image = device
      .allocator()
      .create_image(
        device.raw(),
        (
          gfx_memory::Type::General,
          hal::memory::Properties::DEVICE_LOCAL,
        ),
        hal::image::Kind::D2(size.width(), size.height(), 1, 1),
        1,
        Format::Rgba8Srgb,
        hal::image::Tiling::Optimal,
        hal::image::Usage::TRANSFER_DST | hal::image::Usage::SAMPLED,
        hal::image::ViewCapabilities::empty(),
      )
      .expect("could not create image");

    cmd.begin();

    cmd.record_raw(|cmd| {
      let barrier = hal::memory::Barrier::Image {
        states: (hal::image::Access::empty(), hal::image::Layout::Undefined)
          ..(
            hal::image::Access::TRANSFER_WRITE,
            hal::image::Layout::TransferDstOptimal,
          ),
        target: image.borrow(),
        range: hal::image::SubresourceRange {
          aspects: hal::format::Aspects::COLOR,
          levels: 0..1,
          layers: 0..1,
        },
      };

      cmd.pipeline_barrier(
        hal::pso::PipelineStage::TOP_OF_PIPE..hal::pso::PipelineStage::TRANSFER,
        hal::memory::Dependencies::empty(),
        &[barrier],
      );

      cmd.copy_buffer_to_image(
        buffer.as_ref(),
        image.borrow(),
        hal::image::Layout::TransferDstOptimal,
        &[hal::command::BufferImageCopy {
          buffer_offset: 0,
          buffer_width: 0,
          buffer_height: 0,
          image_layers: hal::image::SubresourceLayers {
            aspects: hal::format::Aspects::COLOR,
            level: 0,
            layers: 0..1,
          },
          image_offset: hal::image::Offset { x: 0, y: 0, z: 0 },
          image_extent: size.into(),
        }],
      );

      let barrier = hal::memory::Barrier::Image {
        states: (
          hal::image::Access::TRANSFER_WRITE,
          hal::image::Layout::TransferDstOptimal,
        )
          ..(
            hal::image::Access::SHADER_READ,
            hal::image::Layout::ShaderReadOnlyOptimal,
          ),
        target: image.borrow(),
        range: hal::image::SubresourceRange {
          aspects: hal::format::Aspects::COLOR,
          levels: 0..1,
          layers: 0..1,
        },
      };

      cmd.pipeline_barrier(
        hal::pso::PipelineStage::TRANSFER..hal::pso::PipelineStage::FRAGMENT_SHADER,
        hal::memory::Dependencies::empty(),
        &[barrier],
      );
    });

    cmd.finish();

    let queue = queue.raw_mut();

    unsafe {
      queue.submit_raw(
        hal::queue::RawSubmission {
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
