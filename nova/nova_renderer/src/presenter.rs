// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::backend::Backend;
use super::device::{Device, DeviceExt, QueueExt};
use super::sync::Semaphore;
use super::texture::{self, RawTexture, RawTextureView, TextureFormat};
use super::Gpu;
use nova_math::Size;
use nova_window::Window;
use std::cmp;

use gfx_hal::Surface as SurfaceExt;
use gfx_hal::Swapchain as SwapchainExt;

type Surface = <Backend as gfx_hal::Backend>::Surface;
type Swapchain = <Backend as gfx_hal::Backend>::Swapchain;

pub struct Presenter {
  size: Size<u32>,
  surface: Surface,
  queue_index: usize,
  swapchain: Option<Swapchain>,
  backbuffers: Vec<Backbuffer>,
  backbuffer_index: Option<usize>,
}

impl Presenter {
  pub fn new(window: &Window, gpu: &Gpu) -> Presenter {
    let surface = gpu.backend().create_surface(window.raw());

    let queue_index = gpu
      .queue_families()
      .iter()
      .position(|f| surface.supports_queue_family(f))
      .expect("The graphics device does not support presentation to the window surface.");

    Presenter {
      size: window.size(),
      surface,
      queue_index,
      swapchain: None,
      backbuffers: Vec::new(),
      backbuffer_index: None,
    }
  }

  pub fn begin(&mut self, gpu: &Gpu, signal_semaphore: &Semaphore) {
    for _ in 0..5 {
      if self.swapchain.is_none() {
        self.create_swapchain(gpu);
      }

      let result = unsafe {
        self.swapchain
          .as_mut()
          .unwrap()
          .acquire_image(!0, gfx_hal::FrameSync::Semaphore(signal_semaphore))
      };

      match result {
        Ok(index) => {
          self.backbuffer_index = Some(index as usize);
          return;
        }

        Err(gfx_hal::AcquireError::SurfaceLost(_)) => {
          panic!("Surface lost.");
        }

        Err(_) => {
          self.destroy_swapchain(gpu.device());
        }
      }
    }

    panic!("Swapchain was repeatedly out of date.");
  }

  pub fn backbuffer(&self) -> &Backbuffer {
    &self.backbuffers[self
      .backbuffer_index
      .expect("Presenter::image called before Presenter::begin.")]
  }

  pub fn finish(&mut self, gpu: &mut Gpu, wait_for: &Semaphore) {
    let backbuffer_index = self
      .backbuffer_index
      .take()
      .expect("Presenter::finish called before Presenter::begin.");

    let swapchain = self.swapchain.as_ref().expect("Swapchain not created.");

    let result = unsafe {
      gpu.queue_mut(self.queue_index)
        .present(Some((swapchain, backbuffer_index as u32)), Some(wait_for))
    };

    if result.is_err() {
      self.destroy_swapchain(gpu.device());
    }
  }

  fn create_swapchain(&mut self, gpu: &Gpu) {
    const FORMAT: TextureFormat = TextureFormat::Bgra8Unorm;

    let (capabilities, _, _, _) = self.surface.compatibility(gpu.physical_device());

    let extent = gfx_hal::window::Extent2D {
      width: cmp::max(
        capabilities.extents.start.width,
        cmp::min(capabilities.extents.end.width, self.size.width),
      ),
      height: cmp::max(
        capabilities.extents.start.height,
        cmp::min(capabilities.extents.end.height, self.size.height),
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
      gpu.device()
        .create_swapchain(&mut self.surface, config, None)
        .expect("Could not create swapchain")
    };

    self.swapchain = swapchain.into();
    self.size = Size::new(extent.width, extent.height);

    match backbuffers {
      gfx_hal::Backbuffer::Images(raw_images) => {
        for raw_image in raw_images {
          let raw_view = texture::create_view(gpu.device(), &raw_image, FORMAT);

          self.backbuffers.push(Backbuffer {
            raw_image,
            raw_view,
            size: self.size,
          });
        }
      }

      // I think this only happens with OpenGL, which isn't supported.
      _ => panic!("Device created framebuffer objects."),
    };
  }

  pub fn destroy(mut self, device: &Device) {
    self.destroy_swapchain(device);
  }

  fn destroy_swapchain(&mut self, device: &Device) {
    device
      .wait_idle()
      .expect("Could not wait for graphics device to be idle");

    for backbuffer in self.backbuffers.drain(..) {
      backbuffer.destroy(device);
    }

    if let Some(swapchain) = self.swapchain.take() {
      unsafe {
        device.destroy_swapchain(swapchain);
      }
    }
  }
}

pub struct Backbuffer {
  #[allow(dead_code)]
  pub(crate) raw_image: RawTexture,
  pub(crate) raw_view: RawTextureView,
  pub(crate) size: Size<u32>,
}

impl Backbuffer {
  fn destroy(self, device: &Device) {
    unsafe {
      device.destroy_image_view(self.raw_view);
    }
  }
}
