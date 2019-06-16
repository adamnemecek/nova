// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::backend;
use crate::{Context, QueueId};
use nova_math::Size;
use nova_window as window;

pub struct Surface {
  size: Size<f64>,
  surface: backend::Surface,
  present_queue_id: QueueId,
  resized: bool,

  context: Context,
}

impl Surface {
  pub fn new(context: &Context, window: &window::Handle) -> Self {
    let size = window.size();
    let surface = context.backend.create_surface(window.as_ref());
    let present_queue_id = context.queues.find_present_queue(&surface);

    Self {
      size,
      surface,
      present_queue_id,
      resized: false,

      context: context.clone(),
    }
  }

  pub fn set_size(&mut self, size: Size<f64>) {
    if size != self.size {
      self.size = size;
      self.resized = true;
    }
  }
}

/*
  pub fn acquire_backbuffer(
    &mut self,
    gpu: &Gpu,
    images: &mut Images,
    signal_ready: &Semaphore,
  ) -> Option<Backbuffer> {
    if self.resized {
      self.resized = false;
      self.destroy_swapchain(gpu, images);
    }

    if self.swapchain.is_none() {
      self.create_swapchain(gpu, images);
    }

    let swapchain = match self.swapchain.as_mut() {
      Some(s) => s,
      None => return None,
    };

    let result =
      unsafe { swapchain.acquire_image(!0, gfx_hal::FrameSync::Semaphore(signal_ready.as_hal())) };

    match result {
      Ok(index) => Some(Backbuffer {
        index,
        image_id: self.swapchain_image_ids[index as usize],
      }),

      _ => None,
    }
  }

  pub fn present_backbuffer<'a, W, Wi>(
    &'a mut self,
    queues: &mut CommandQueues,
    backbuffer: Backbuffer,
    wait_semaphores: W,
  ) where
    W: IntoIterator<Item = &'a Wi>,
    Wi: 'a + Borrow<Semaphore>,
  {
    let swapchain = match self.swapchain.as_ref() {
      Some(s) => s,
      None => return,
    };

    use std::iter;

    let wait_semaphores = wait_semaphores
      .into_iter()
      .map(Borrow::borrow)
      .map(Semaphore::as_hal);

    let result = unsafe {
      queues
        .get_mut(&self.queue_family)
        .present(iter::once((swapchain, backbuffer.index)), wait_semaphores)
    };

    if result.is_err() {
      self.resized = true;
    }
  }

  fn create_swapchain(&mut self, gpu: &Gpu, images: &mut Images) {
    let (capabilities, _, _, _) = self.surface.compatibility(&gpu.adapter.physical_device);

    let extent = gfx_hal::window::Extent2D {
      width: clamp(
        self.size.width,
        capabilities.extents.start.width..capabilities.extents.end.width,
      ),
      height: clamp(
        self.size.height,
        capabilities.extents.start.height..capabilities.extents.end.height,
      ),
    };

    let image_count = match capabilities.image_count.end {
      0 => 2, // Any number of images is allowed. Only need two.
      x => cmp::min(x, 2),
    };

    let config = gfx_hal::SwapchainConfig {
      present_mode: gfx_hal::window::PresentMode::Fifo,
      format: FORMAT,
      extent,
      image_count,
      image_layers: 1,
      image_usage: gfx_hal::image::Usage::COLOR_ATTACHMENT,
      composite_alpha: gfx_hal::window::CompositeAlpha::Opaque,
    };

    let (swapchain, backbuffers) = unsafe {
      gpu
        .device
        .create_swapchain(&mut self.surface, config, None)
        .expect("Could not create swapchain")
    };

    self.swapchain = Some(swapchain);
    self.size = Size::new(extent.width, extent.height);

    match backbuffers {
      gfx_hal::Backbuffer::Images(imgs) => {
        for img in imgs {
          let image = Image::new_view(&gpu, img, FORMAT, self.size);
          let id = images.insert(image);

          images.transition_image(
            id,
            PipelineStage::COLOR_ATTACHMENT_OUTPUT,
            ImageAccess::empty()..ImageAccess::empty(),
            ImageLayout::Undefined..ImageLayout::Present,
          );

          self.swapchain_image_ids.push(id);
        }
      }

      // I think this only happens with OpenGL, which isn't supported.
      _ => panic!("Device created framebuffer objects."),
    };
  }

  pub fn destroy(mut self, gpu: &Gpu, images: &mut Images) {
    self.destroy_swapchain(gpu, images);
  }

  fn destroy_swapchain(&mut self, gpu: &Gpu, images: &mut Images) {
    for image_id in self.swapchain_image_ids.drain(..) {
      images.destroy_image(gpu, image_id);
    }

    if let Some(swapchain) = self.swapchain.take() {
      unsafe { gpu.device.destroy_swapchain(swapchain) };
    }
  }
}

pub struct Backbuffer {
  index: u32,
  image_id: ImageId,
}

impl Backbuffer {
  pub fn image_id(&self) -> ImageId {
    self.image_id
  }
}

quick_error! {
  #[derive(Debug)]
  pub enum CreateSurfaceError {
    PresentNotSupported {
      display("the graphics device does not support presentation to this window")
    }
  }
}

*/
