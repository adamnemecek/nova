use crate::graphics::backend;
use crate::graphics::device::{Device, Semaphore};
use crate::graphics::hal::prelude::*;
use crate::graphics::image::{self, Image};
use crate::graphics::rendering::RenderPass;
use crate::math::algebra::Vector2;
use crate::utils::{quick_error, Droppable};
use smallvec::SmallVec;
use std::cmp;
use std::sync::Arc;

pub struct Swapchain {
  device: Arc<Device>,
  raw: Droppable<backend::Swapchain>,
  images: SmallVec<[Arc<Image>; 3]>,
  size: Vector2<f32>,
}

impl Swapchain {
  pub fn new(render_pass: &RenderPass, surface: &mut backend::Surface, size: Vector2<f32>) -> Self {
    let device = render_pass.device();
    let (caps, _, modes) = surface.compatibility(&device.adapter().physical_device);

    let extent = gfx_hal::window::Extent2D {
      width: cmp::max(
        caps.extents.start.width,
        cmp::min(size.x.round() as u32, caps.extents.end.width),
      ),
      height: cmp::max(
        caps.extents.start.height,
        cmp::min(size.y.round() as u32, caps.extents.end.height),
      ),
    };

    let present_mode = select_present_mode(modes);

    let image_count = if present_mode == gfx_hal::window::PresentMode::Mailbox {
      cmp::max(caps.image_count.start, cmp::min(3, caps.image_count.end))
    } else {
      caps.image_count.start
    };

    let config = gfx_hal::SwapchainConfig {
      present_mode,
      format: render_pass.format(),
      extent,
      image_count,
      image_layers: 1,
      image_usage: gfx_hal::image::Usage::COLOR_ATTACHMENT,
    };

    let (raw, backbuffer) = device
      .raw()
      .create_swapchain(surface, config, None)
      .expect("could not create swapchain");

    let mut swapchain = Swapchain {
      device: device.clone(),
      raw: raw.into(),
      images: SmallVec::new(),
      size: Vector2::new(extent.width as f32, extent.height as f32),
    };

    match backbuffer {
      gfx_hal::Backbuffer::Images(images) => {
        for image in images {
          swapchain.images.push(Arc::new(Image::from_raw(
            device,
            image::Backing::Swapchain(image),
            render_pass.format(),
            Vector2::new(extent.width, extent.height),
          )));
        }
      }

      _ => panic!("device created framebuffer objects"),
    };

    swapchain
  }

  pub fn raw(&self) -> &backend::Swapchain {
    &self.raw
  }

  pub fn images(&self) -> impl Iterator<Item = &Arc<Image>> {
    self.images.iter()
  }

  pub fn size(&self) -> Vector2<f32> {
    self.size
  }

  pub fn acquire_image(&mut self, semaphore: &Semaphore) -> Result<usize, AcquireImageError> {
    let index = self
      .raw
      .acquire_image(!0, gfx_hal::FrameSync::Semaphore(semaphore.raw()))
      .map_err(|err| match err {
        gfx_hal::AcquireError::OutOfDate => AcquireImageError::OutOfDate,
        gfx_hal::AcquireError::NotReady => panic!("Swapchain::acquire_image timed out."),
        gfx_hal::AcquireError::SurfaceLost(_) => panic!("Surface lost."),
      })?;

    Ok(index as usize)
  }
}

impl Drop for Swapchain {
  fn drop(&mut self) {
    let device = self.device.raw();

    self.images.clear();

    if let Some(swapchain) = self.raw.take() {
      device.destroy_swapchain(swapchain);
    }
  }
}

fn select_present_mode(modes: Vec<gfx_hal::window::PresentMode>) -> gfx_hal::window::PresentMode {
  if modes.contains(&gfx_hal::window::PresentMode::Mailbox) {
    gfx_hal::window::PresentMode::Mailbox
  } else if modes.contains(&gfx_hal::window::PresentMode::Immediate) {
    gfx_hal::window::PresentMode::Immediate
  } else {
    gfx_hal::window::PresentMode::Fifo
  }
}

quick_error! {
  #[derive(Debug)]
  pub enum AcquireImageError {
    OutOfDate {
      display("The swapchain is out of date and must be recreated.")
    }
  }
}
