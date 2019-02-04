// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::surface::{RawSurfaceExt, Surface};
use super::Window;
use crate::ecs;
use crate::graphics::{self, RawDeviceExt, RawQueueExt};
use crate::math::Size;
use crate::utils::Droppable;
use std::cmp;
use std::sync::Arc;

use gfx_hal::Swapchain as RawSwapchainExt;

type RawSwapchain = <graphics::Backend as gfx_hal::Backend>::Swapchain;

const MAX_FRAMES: usize = 3;

pub struct Presenter {
  images: Vec<Arc<graphics::Image>>,
  image_index: Option<usize>,
  swapchain: Droppable<RawSwapchain>,
  device: graphics::Device,
  queue_id: graphics::QueueId,
  size: Size<u32>,
}

impl Presenter {
  pub fn new(window: &Window, queues: &graphics::Queues) -> Presenter {
    let queue_id = queues
      .find_queue_raw(|family| window.surface.raw().supports_queue_family(family))
      .expect("The graphics device does not support presentation to the window surface.");

    Presenter {
      images: Vec::with_capacity(MAX_FRAMES),
      image_index: None,
      swapchain: Droppable::dropped(),
      device: window.surface.device().clone(),
      queue_id,
      size: window.size,
    }
  }

  pub fn begin(&mut self, res: &mut ecs::Resources, image_ready: &graphics::Semaphore) {
    for _ in 0..5 {
      if self.swapchain.is_dropped() {
        self.create_swapchain(res);
      }

      let result = unsafe {
        self
          .swapchain
          .acquire_image(!0, gfx_hal::FrameSync::Semaphore(image_ready.raw()))
      };

      match result {
        Ok(index) => {
          self.image_index = Some(index as usize);
          return;
        }

        Err(gfx_hal::AcquireError::SurfaceLost(_)) => {
          panic!("Surface lost.");
        }

        Err(_) => {
          self.destroy_swapchain();
        }
      }
    }

    panic!("Swapchain was repeatedly out of date.");
  }

  pub fn image(&self) -> &Arc<graphics::Image> {
    &self.images[self
      .image_index
      .expect("Presenter::image called before Presenter::begin.")]
  }

  pub fn finish(&mut self, res: &mut ecs::Resources, wait_for: Option<&graphics::Semaphore>) {
    let image_index = self
      .image_index
      .take()
      .expect("Presenter::finish called before Presenter::begin.");

    let mut queues = res.fetch_mut::<graphics::Queues>();

    let result = unsafe {
      queues.raw_mut(self.queue_id).present(
        Some((&self.swapchain, image_index as u32)),
        wait_for.map(|sem| sem.raw()),
      )
    };

    if result.is_err() {
      self.destroy_swapchain();
    }
  }

  fn create_swapchain(&mut self, res: &mut ecs::Resources) {
    let mut surface = res.fetch_mut::<Surface>();

    let capabilities = surface.capabilities();
    let format = gfx_hal::format::Format::Bgra8Unorm;

    let extent = gfx_hal::window::Extent2D {
      width: cmp::max(
        capabilities.extents.start.width,
        cmp::min(capabilities.extents.end.width, self.size.width()),
      ),
      height: cmp::max(
        capabilities.extents.start.height,
        cmp::min(capabilities.extents.end.height, self.size.height()),
      ),
    };

    let image_count = match capabilities.image_count.end {
      0 => 2, // Any number of images is allowed. Only need two.
      x => cmp::min(x, 2),
    };

    let config = gfx_hal::SwapchainConfig {
      present_mode: gfx_hal::window::PresentMode::Fifo,
      format,
      extent,
      image_count,
      image_layers: 1,
      image_usage: gfx_hal::image::Usage::COLOR_ATTACHMENT,
      composite_alpha: gfx_hal::window::CompositeAlpha::Opaque,
    };

    let (swapchain, backbuffers) = unsafe {
      self
        .device
        .raw()
        .create_swapchain(surface.raw_mut(), config, None)
        .expect("Could not create swapchain")
    };

    self.swapchain = swapchain.into();
    self.size = extent.into();

    match backbuffers {
      gfx_hal::Backbuffer::Images(raw_images) => {
        for raw in raw_images {
          let image = graphics::Image::from_raw_image(&self.device, raw, format, self.size);

          self.images.push(Arc::new(image));
        }
      }

      // I think this only happens with OpenGL, which isn't supported.
      _ => panic!("Device created framebuffer objects."),
    };
  }

  fn destroy_swapchain(&mut self) {
    self
      .device
      .raw()
      .wait_idle()
      .expect("Could not wait for graphics device to be idle");

    self.images.clear();

    if let Some(swapchain) = self.swapchain.take() {
      unsafe {
        self.device.raw().destroy_swapchain(swapchain);
      }
    }
  }
}

impl Drop for Presenter {
  fn drop(&mut self) {
    self.destroy_swapchain();
  }
}
