// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Window;
use crate::ecs;
use crate::graphics;
use crate::graphics::{RawDeviceExt, RawQueueExt};
use crate::math::Size;
use crate::utils::Droppable;
use std::cmp;
use std::sync::{Arc, Weak};

use gfx_hal::Surface as RawSurfaceExt;
use gfx_hal::Swapchain as RawSwapchainExt;

type RawSurface = <graphics::Backend as gfx_hal::Backend>::Surface;
type RawSwapchain = <graphics::Backend as gfx_hal::Backend>::Swapchain;

pub struct Surface {
  images: Vec<Arc<graphics::Image>>,
  swapchain: Droppable<RawSwapchain>,
  raw: RawSurface,
  device: graphics::DeviceHandle,
  size: Size<u32>,
  present_queue: Option<graphics::QueueIndex>,
}

impl Surface {
  fn new(window: &Window, device: &graphics::DeviceHandle) -> Self {
    let surface = device.backend().create_surface(&window.raw);

    Surface {
      raw: surface,
      device: device.clone(),
      size: window.size,
      swapchain: Droppable::dropped(),
      images: Vec::new(),
      present_queue: None,
    }
  }

  pub fn acquire_backbuffer(&mut self, signal_ready: &graphics::Semaphore) -> Backbuffer {
    for _ in 0..5 {
      self.ensure_swapchain();

      let result = unsafe {
        self
          .swapchain
          .acquire_image(!0, gfx_hal::FrameSync::Semaphore(signal_ready.raw()))
      };

      match result {
        Ok(index) => {
          let index = index as usize;
          let image = Arc::downgrade(&self.images[index]);

          return Backbuffer { image, index };
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

  pub fn present_backbuffer(
    &mut self,
    backbuffer: Backbuffer,
    queues: &mut graphics::Queues,
    wait_for: &graphics::Semaphore,
  ) {
    debug_assert!(
      Arc::ptr_eq(
        &self.images[backbuffer.index],
        &backbuffer.image.upgrade().unwrap(),
      ),
      "Cannot present backbuffers from other surfaces."
    );

    let queue_index = match self.present_queue {
      Some(i) => i,

      None => {
        self.present_queue = queues.find_queue_raw(|family| self.raw.supports_queue_family(family));

        self
          .present_queue
          .expect("The graphics device does not support presentation to the window surface.")
      }
    };

    let result = unsafe {
      queues.raw_mut(queue_index).present(
        Some((&self.swapchain, backbuffer.index as u32)),
        Some(wait_for.raw()),
      )
    };

    if result.is_err() {
      self.destroy_swapchain();
    }
  }

  fn ensure_swapchain(&mut self) {
    if self.swapchain.is_dropped() {
      self.create_swapchain();
    }
  }

  fn create_swapchain(&mut self) {
    let (capabilities, _, _, _) = self.raw.compatibility(self.device.raw_physical());

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
        .create_swapchain(&mut self.raw, config, None)
        .expect("Could not create swapchain")
    };

    self.swapchain = swapchain.into();
    self.size = extent.into();

    match backbuffers {
      gfx_hal::Backbuffer::Images(raw_images) => {
        for raw in raw_images {
          let image = graphics::Image::from_raw_image(&self.device, raw, format);

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

impl Drop for Surface {
  fn drop(&mut self) {
    self.destroy_swapchain();
  }
}

pub struct MaintainSurface;

impl<'a> ecs::System<'a> for MaintainSurface {
  type SystemData = (
    ecs::ReadResource<'a, Window>,
    ecs::WriteResource<'a, Surface>,
  );

  fn setup(&mut self, res: &mut ecs::Resources) {
    res.insert({
      let device = res.fetch();
      let window = res.fetch();

      Surface::new(&window, &device)
    });
  }

  fn run(&mut self, (window, mut surface): Self::SystemData) {
    if surface.size != window.size {
      surface.destroy_swapchain();
    }
  }
}

pub struct Backbuffer {
  image: Weak<graphics::Image>,
  index: usize,
}
