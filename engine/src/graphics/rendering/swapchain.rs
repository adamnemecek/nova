use super::backend;
use super::prelude::*;
use super::{Device, RenderPass};
use smallvec::SmallVec;
use std::cmp;
use std::sync::Arc;

pub struct Swapchain {
  width: u32,
  height: u32,
  images: SmallVec<[Image; 3]>,
  raw: Option<backend::Swapchain>,
  device: Arc<Device>,
}

struct Image {
  framebuffer: backend::Framebuffer,
  view: backend::ImageView,
  _raw: backend::Image,
}

impl Swapchain {
  pub fn new(device: &Arc<Device>) -> Self {
    Swapchain {
      width: 0,
      height: 0,
      images: SmallVec::new(),
      raw: None,
      device: device.clone(),
    }
  }

  pub fn create(&mut self, render_pass: &RenderPass, width: u32, height: u32) {
    if !self.is_destroyed() {
      panic!("swapchain is already created");
    }

    let mut surface = self.device.surface.lock().unwrap();
    let (caps, _, modes) = surface.compatibility(&self.device.adapter.physical_device);
    let device = &self.device.raw;

    let extent = gfx_hal::window::Extent2D {
      width: cmp::max(
        caps.extents.start.width,
        cmp::min(width, caps.extents.end.width),
      ),
      height: cmp::max(
        caps.extents.start.height,
        cmp::min(height, caps.extents.end.height),
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

    let (swapchain, backbuffer) = self.device.raw.create_swapchain(&mut surface, config, None);

    self.raw = Some(swapchain);
    self.width = extent.width;
    self.height = extent.height;

    let images = match backbuffer {
      gfx_hal::Backbuffer::Images(images) => images,
      _ => panic!("device created framebuffer objects"),
    };

    self.images.extend(images.into_iter().map(|image| {
      let view = device
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
        .create_framebuffer(render_pass.raw(), Some(&view), extent.to_extent())
        .unwrap();

      Image {
        framebuffer,
        view,
        _raw: image,
      }
    }));
  }

  pub fn raw_mut(&mut self) -> &mut backend::Swapchain {
    self.raw.as_mut().expect("swapchain is destroyed")
  }

  pub fn framebuffer(&self, index: u32) -> &backend::Framebuffer {
    &self.images[index as usize].framebuffer
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn is_destroyed(&self) -> bool {
    self.raw.is_none()
  }

  pub fn destroy(&mut self) {
    let device = &self.device.raw;

    for image in self.images.drain() {
      device.destroy_framebuffer(image.framebuffer);
      device.destroy_image_view(image.view);
    }

    if let Some(swapchain) = self.raw.take() {
      device.destroy_swapchain(swapchain);
    }
  }
}

impl Drop for Swapchain {
  fn drop(&mut self) {
    self.destroy();
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
