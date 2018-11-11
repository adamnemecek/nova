// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Backing, Format, Image, Source};
use crate::graphics::buffer::{self, Buffer};
use crate::graphics::commands::{self, CommandPool, Commands};
use crate::graphics::device::{self, Device};
use crate::graphics::hal::prelude::*;
use gfx_memory::Factory;
use std::borrow::Borrow;
use std::iter;
use std::sync::Arc;

pub struct Loader {
  device: Arc<Device>,
  command_pool: Arc<CommandPool>,
}

impl Loader {
  pub fn new(queue: &device::Queue) -> Self {
    Loader {
      device: queue.device().clone(),
      command_pool: CommandPool::new(&queue),
    }
  }

  pub fn load(&mut self, queue: &mut device::Queue, source: &Source) -> Image {
    assert!(
      self.command_pool.queue_family_id() == queue.family_id(),
      "Images must be loaded with a queue in family {}.",
      self.command_pool.queue_family_id()
    );

    let device = &self.device;
    let mut cmd = Commands::new(&self.command_pool, commands::Level::Primary);

    let size = source.size();

    let mut buffer = Buffer::new(device, source.bytes().len(), buffer::Usage::TRANSFER_SRC);

    buffer.write(source.bytes());

    let image = device
      .allocator()
      .create_image(
        device.raw(),
        (
          gfx_memory::Type::General,
          hal::memory::Properties::DEVICE_LOCAL,
        ),
        hal::image::Kind::D2(size.x, size.y, 1, 1),
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
          image_extent: hal::image::Extent {
            width: size.x,
            height: size.y,
            depth: 1,
          },
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
