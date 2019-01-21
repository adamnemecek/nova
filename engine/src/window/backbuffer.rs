// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Window;
use crate::ecs;
use crate::graphics::device::{QueueExt, QueueFamilyExt};
use crate::graphics::{self, DeviceExt, Graphics};
use crate::log::trace;
use crate::math::Size;
use crate::utils::Ring;
use gfx_hal::Surface as SurfaceExt;
use gfx_hal::SurfaceCapabilities;
use gfx_hal::Swapchain as SwapchainExt;
use std::cmp;

type Image = <graphics::Backend as gfx_hal::Backend>::Image;
type ImageView = <graphics::Backend as gfx_hal::Backend>::ImageView;
type Semaphore = <graphics::Backend as gfx_hal::Backend>::Semaphore;
type Surface = <graphics::Backend as gfx_hal::Backend>::Surface;
type Swapchain = <graphics::Backend as gfx_hal::Backend>::Swapchain;

pub struct Backbuffer {
  size: Size<u32>,
  capabilities: SurfaceCapabilities,
  images: Vec<Image>,
  image_views: Vec<ImageView>,
  image_index: Option<usize>,
  swapchain: Option<Swapchain>,
  semaphores: Ring<Semaphore>,
  queue_index: usize,
  surface: Surface,
  device: graphics::DeviceHandle,
}

impl Backbuffer {
  fn new(res: &mut ecs::Resources) -> Self {
    let gfx = res
      .try_fetch::<Graphics>()
      .expect("Cannot set up window backbuffer before graphics.");

    let device = gfx.device().clone();

    let window = res
      .try_fetch::<Window>()
      .expect("Cannot set up window backbuffer before window.");

    let surface = gfx.instance().create_surface(window.handle());

    let queue_index = device
      .queue_families()
      .iter()
      .find(|f| surface.supports_queue_family(f))
      .expect("No graphics device queue family supports presentation to the window surface.")
      .id()
      .0;

    let (capabilities, _, _, _) = surface.compatibility(device.physical());

    Backbuffer {
      device: device.clone(),
      surface,
      queue_index,
      capabilities,
      size: window.size(),
      images: Vec::new(),
      image_views: Vec::new(),
      image_index: None,
      swapchain: None,
      semaphores: Ring::new(2, |_| {
        device
          .create_semaphore()
          .expect("Could not create backbuffer semaphore")
      }),
    }
  }

  fn create_swapchain(&mut self) {
    let format = gfx_hal::format::Format::Bgra8Unorm;

    let extent = gfx_hal::window::Extent2D {
      width: cmp::max(
        self.capabilities.extents.start.width,
        cmp::min(self.capabilities.extents.end.width, self.size.width()),
      ),
      height: cmp::max(
        self.capabilities.extents.start.height,
        cmp::min(self.capabilities.extents.end.height, self.size.height()),
      ),
    };

    let image_count = match self.capabilities.image_count.end {
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
        .create_swapchain(&mut self.surface, config, None)
        .expect("Could not create swapchain")
    };

    self.swapchain = Some(swapchain);
    self.size = extent.into();

    match backbuffers {
      gfx_hal::Backbuffer::Images(images) => {
        for image in images {
          let image_view = create_image_view(&self.device, &image, format);

          self.images.push(image);
          self.image_views.push(image_view);
        }
      }

      // I think this only happens with OpenGL, which isn't supported.
      _ => panic!("Device created framebuffer objects."),
    };
  }

  fn destroy_swapchain(&mut self) {
    if let Some(swapchain) = self.swapchain.take() {
      self
        .device
        .wait_idle()
        .expect("Error on device.wait_idle()");

      for view in self.image_views.drain(..) {
        unsafe {
          self.device.destroy_image_view(view);
        }
      }

      unsafe {
        self.device.destroy_swapchain(swapchain);
      }
    }
  }

  fn acquire_image(&mut self) -> Result<u32, gfx_hal::AcquireError> {
    let swapchain = self.swapchain.as_mut().unwrap();
    let semaphore = self.semaphores.current();

    unsafe { swapchain.acquire_image(!0, gfx_hal::FrameSync::Semaphore(semaphore)) }
  }
}

fn create_image_view(
  device: &graphics::DeviceHandle,
  image: &Image,
  format: gfx_hal::format::Format,
) -> ImageView {
  unsafe {
    device
      .create_image_view(
        image,
        gfx_hal::image::ViewKind::D2,
        format,
        gfx_hal::format::Swizzle::NO,
        gfx_hal::image::SubresourceRange {
          aspects: gfx_hal::format::Aspects::COLOR,
          levels: 0..1,
          layers: 0..1,
        },
      )
      .expect("Could not create image view")
  }
}

pub fn acquire_backbuffer() -> AcquireBackbuffer {
  AcquireBackbuffer
}

pub struct AcquireBackbuffer;

impl<'a> ecs::System<'a> for AcquireBackbuffer {
  type SystemData = (
    ecs::ReadResource<'a, Window>,
    ecs::WriteResource<'a, Backbuffer>,
  );

  fn setup(&mut self, res: &mut ecs::Resources) {
    if !res.has_value::<Backbuffer>() {
      let backbuffer = Backbuffer::new(res);

      res.insert(backbuffer);
    }
  }

  fn run(&mut self, (window, mut backbuffer): Self::SystemData) {
    trace!("Acquiring.");

    backbuffer.semaphores.advance();

    for _ in 0..2 {
      if backbuffer.size != window.size() {
        backbuffer.destroy_swapchain();
      }

      if backbuffer.swapchain.is_none() {
        backbuffer.create_swapchain();
      }

      match backbuffer.acquire_image() {
        Ok(index) => {
          backbuffer.image_index = Some(index as usize);
          return;
        }

        Err(gfx_hal::AcquireError::OutOfDate) => {
          backbuffer.destroy_swapchain();
        }

        Err(err) => {
          panic!("Could not acquire window backbuffer: {:?}.", err);
        }
      };
    }

    panic!("Swapchain was repeatedly out of date.");
  }
}

pub fn present_backbuffer() -> PresentBackbuffer {
  PresentBackbuffer
}

pub struct PresentBackbuffer;

impl<'a> ecs::System<'a> for PresentBackbuffer {
  type SystemData = (
    ecs::ReadResource<'a, Backbuffer>,
    ecs::WriteResource<'a, Graphics>,
  );

  fn setup(&mut self, res: &mut ecs::Resources) {
    if !res.has_value::<Backbuffer>() {
      let backbuffer = Backbuffer::new(res);

      res.insert(backbuffer);
    }
  }

  fn run(&mut self, (backbuffer, mut graphics): Self::SystemData) {
    trace!("Presenting.");
    if let Some(ref swapchain) = backbuffer.swapchain {
      if let Some(index) = backbuffer.image_index {
        let queue = graphics.queue_mut(backbuffer.queue_index);

        unsafe {
          let _ = queue.present(
            Some((swapchain, index as u32)),
            Some(backbuffer.semaphores.current()),
          );

          let _ = graphics.device().wait_idle();
        }
      }
    }
  }
}
