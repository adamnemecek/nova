use crate::graphics::device::{Device, Semaphore};
use crate::graphics::hal::*;
use crate::graphics::rendering::RenderPass;
use crate::math::algebra::Vector2;
use crate::utils::{quick_error, Chain, Droppable};
use smallvec::SmallVec;
use std::cmp;
use std::sync::Arc;

pub struct Swapchain {
  device: Arc<Device>,
  raw: Droppable<backend::Swapchain>,
  images: SmallVec<[backend::Image; 3]>,
  image_views: SmallVec<[backend::ImageView; 3]>,
  framebuffers: SmallVec<[Arc<Framebuffer>; 3]>,
  semaphores: Chain<Semaphore>,
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

    let width = extent.width as i16;
    let height = extent.height as i16;

    let mut swapchain = Swapchain {
      device: device.clone(),
      raw: raw.into(),
      images: SmallVec::new(),
      image_views: SmallVec::new(),
      framebuffers: SmallVec::new(),
      semaphores: Chain::allocate(3, |_| Semaphore::new(device)),
      size: Vector2::new(extent.width as f32, extent.height as f32),
    };

    match backbuffer {
      gfx_hal::Backbuffer::Images(images) => {
        for image in images {
          let view = device
            .raw()
            .create_image_view(
              &image,
              gfx_hal::image::ViewKind::D2,
              render_pass.format(),
              gfx_hal::format::Swizzle::NO,
              gfx_hal::image::SubresourceRange {
                aspects: gfx_hal::format::Aspects::COLOR,
                levels: 0..1,
                layers: 0..1,
              },
            )
            .expect("could not create image view");

          let framebuffer = device
            .raw()
            .create_framebuffer(render_pass.raw(), Some(&view), extent.to_extent())
            .expect("could not create framebuffer");

          swapchain.images.push(image);
          swapchain.image_views.push(view);

          swapchain.framebuffers.push(Arc::new(Framebuffer {
            index: swapchain.framebuffers.len() as u32,
            raw: framebuffer,
            width,
            height,
          }));
        }
      }

      _ => panic!("device created framebuffer objects"),
    };

    swapchain
  }

  pub fn raw(&self) -> &backend::Swapchain {
    &self.raw
  }

  pub fn raw_mut(&mut self) -> &mut backend::Swapchain {
    &mut self.raw
  }

  pub fn size(&self) -> Vector2<f32> {
    self.size
  }

  pub fn acquire_framebuffer(
    &mut self,
  ) -> Result<(Arc<Framebuffer>, &Semaphore), AcquireFramebufferError> {
    let semaphore = self.semaphores.next();

    let index = self
      .raw
      .acquire_image(!0, gfx_hal::FrameSync::Semaphore(semaphore.raw()))
      .map_err(|err| match err {
        gfx_hal::AcquireError::OutOfDate => AcquireFramebufferError::SwapchainOutOfDate,
        _ => panic!("could not acquire framebuffer"),
      })?;

    Ok((self.framebuffers[index as usize].clone(), semaphore))
  }
}

impl Drop for Swapchain {
  fn drop(&mut self) {
    let device = self.device.raw();

    for framebuffer in self.framebuffers.drain() {
      if let Ok(framebuffer) = Arc::try_unwrap(framebuffer) {
        device.destroy_framebuffer(framebuffer.raw);
      } else {
        panic!("swapchain framebuffer still in use");
      }
    }

    for view in self.image_views.drain() {
      device.destroy_image_view(view);
    }

    if let Some(swapchain) = self.raw.take() {
      device.destroy_swapchain(swapchain);
    }
  }
}

pub struct Framebuffer {
  raw: backend::Framebuffer,
  index: u32,
  width: i16,
  height: i16,
}

impl Framebuffer {
  pub fn raw(&self) -> &backend::Framebuffer {
    &self.raw
  }

  pub fn index(&self) -> u32 {
    self.index
  }

  pub fn width(&self) -> i16 {
    self.width
  }

  pub fn height(&self) -> i16 {
    self.height
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
  pub enum AcquireFramebufferError {
    SwapchainOutOfDate {
      display("The given swapchain is out of date and must be recreated.")
    }
  }
}
