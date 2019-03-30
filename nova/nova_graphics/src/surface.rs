// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::gpu::queues::{GpuQueueId, GpuQueues};
use crate::gpu::{self, Gpu};
use crate::images::{Image, ImageAccess, ImageFormat, ImageId, ImageLayout, Images};
use crate::pipelines::PipelineStage;
use crate::sync::Semaphore;
use crate::Backend;
use gfx_hal::queue::RawCommandQueue as _;
use gfx_hal::{Device as _, Surface as _, Swapchain as _};
use nova_core::math::{clamp, Size};
use nova_core::quick_error;
use nova_core::resources::Resources;
use std::borrow::Borrow;
use std::cmp;
use winit::Window;

pub(crate) type HalSurface = <Backend as gfx_hal::Backend>::Surface;
type HalSwapchain = <Backend as gfx_hal::Backend>::Swapchain;

const FORMAT: ImageFormat = ImageFormat::Bgra8Unorm;

pub struct Surface {
  surface: HalSurface,
  swapchain: Option<HalSwapchain>,
  swapchain_image_ids: Vec<ImageId>,
  present_queue_id: GpuQueueId,
  size: Size<u32>,
  resized: bool,
}

impl Surface {
  pub fn new(res: &Resources, window: &Window) -> Result<Self, CreateSurfaceError> {
    let gpu = gpu::borrow(res);

    let surface = gpu.backend.create_surface(window);

    let present_queue_id = gpu::queues::borrow(res)
      .find(|q| surface.supports_queue_family(&q.family))
      .ok_or(CreateSurfaceError::PresentNotSupported)?;

    let (width, height): (u32, u32) = window
      .get_inner_size()
      .expect("Could not get window size")
      .to_physical(window.get_hidpi_factor())
      .into();

    Ok(Self {
      surface,
      swapchain: None,
      swapchain_image_ids: Vec::with_capacity(3),
      present_queue_id,
      size: Size { width, height },
      resized: false,
    })
  }

  pub fn set_size(&mut self, size: Size<u32>) {
    if size != self.size {
      self.size = size;
      self.resized = true;
    }
  }

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
    queues: &mut GpuQueues,
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
      queues[self.present_queue_id]
        .as_hal_mut()
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
