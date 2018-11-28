// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::device;
use crate::graphics::prelude::*;
use crate::graphics::sync::Semaphore;
use crate::math::Size;
use crate::utils::Droppable;
use crate::window;
use std::cmp;
use std::iter;
use std::sync::{Arc, Weak};

pub struct Surface {
  device: device::Handle,
  surface: Droppable<backend::Surface>,
  size: Size<u32>,
  swapchain: Droppable<backend::Swapchain>,
  backbuffers: Vec<Arc<Backbuffer>>,
  backbuffer_index: Option<usize>,
}

impl Surface {
  pub fn new(device: &device::Handle, window: &window::Handle) -> Surface {
    let surface = device.backend().create_surface(window.as_ref());

    Surface {
      device: device.clone(),
      surface: surface.into(),
      size: window.get_size(),
      swapchain: Droppable::dropped(),
      backbuffers: Vec::new(),
      backbuffer_index: None,
    }
  }

  pub fn device(&self) -> &device::Handle {
    &self.device
  }

  pub fn backbuffer(&self) -> Weak<Backbuffer> {
    let index = self
      .backbuffer_index
      .expect("Surface::begin() must be called before the backbuffer can be accessed.");

    let backbuffer = &self.backbuffers[index];

    Arc::downgrade(&self.backbuffers[index])
  }

  pub fn supports_queue_family(&self, family: &backend::QueueFamily) -> bool {
    self.surface.supports_queue_family(family)
  }

  pub fn begin(&mut self, semaphore: &Semaphore) {
    for _ in 0..5 {
      if self.swapchain.is_dropped() {
        self.create_swapchain();
      }

      let result = self
        .swapchain
        .acquire_image(!0, hal::FrameSync::Semaphore(semaphore.as_ref()));

      match result {
        Ok(index) => {
          self.backbuffer_index = Some(index as usize);
          return;
        }

        Err(hal::AcquireError::OutOfDate) => {
          self.destroy_swapchain();
        }
      };
    }

    panic!("Swapchain was repeatedly out of date.");
  }

  pub fn finish<'a>(&mut self, wait_for: impl IntoIterator<Item = &'a Semaphore>) {
    let index = self
      .backbuffer_index
      .expect("Surface::finish() was called before Surface::begin().");

    let queue = device::get_present_queue(&self);

    queue.lock().raw_mut().present(
      iter::once((self.swapchain.as_ref(), index as u32)),
      wait_for.into_iter().map(AsRef::as_ref),
    );
  }

  fn create_swapchain(&mut self) {
    let (caps, _, _) = self
      .surface
      .compatibility(&self.device.adapter().physical_device);

    let format = hal::format::Format::Bgra8Unorm;
    let extent = caps.current_extent.unwrap_or_else(|| self.size.into());

    let image_count = match caps.image_count.end {
      0 => 2, // Any number of images is allowed. Only need two.
      x => cmp::min(x, 2),
    };

    let config = hal::SwapchainConfig {
      present_mode: hal::window::PresentMode::Fifo,
      format,
      extent,
      image_count,
      image_layers: 1,
      image_usage: hal::image::Usage::COLOR_ATTACHMENT,
    };

    let (swapchain, backbuffers) = self
      .device
      .raw()
      .create_swapchain(&mut self.surface, config, None)
      .expect("Could not create swapchain");

    self.swapchain = swapchain.into();
    self.size = extent.into();

    match backbuffers {
      hal::Backbuffer::Images(images) => {
        for image in images {
          let image_view = create_image_view(&self.device, &image, format);

          self
            .backbuffers
            .push(Arc::new(Backbuffer { image, image_view }));
        }
      }

      // I think this only happens with OpenGL, which isn't supported.
      _ => panic!("Device created framebuffer objects."),
    };
  }

  fn destroy_swapchain(&mut self) {
    if let Some(swapchain) = self.swapchain.take() {
      self.device.wait_idle();

      for backbuffer in self.backbuffers.drain(..) {
        self.device.raw().destroy_image_view(backbuffer.image_view);
      }

      self.device.raw().destroy_swapchain(swapchain);
    }
  }
}

impl Drop for Surface {
  fn drop(&mut self) {
    self.destroy_swapchain();
  }
}

pub struct Backbuffer {
  image: backend::Image,
  image_view: backend::ImageView,
}

fn create_image_view(
  device: &device::Handle,
  image: &backend::Image,
  format: hal::format::Format,
) -> backend::ImageView {
  device
    .raw()
    .create_image_view(
      image,
      hal::image::ViewKind::D2,
      format,
      hal::format::Swizzle::NO,
      hal::image::SubresourceRange {
        aspects: hal::format::Aspects::COLOR,
        levels: 0..1,
        layers: 0..1,
      },
    )
    .expect("Could not create image view")
}
