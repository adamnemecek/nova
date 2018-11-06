use super::*;
use smallvec::SmallVec;
use std::cmp;
use std::sync::Arc;

pub struct Swapchain {
  device: Arc<Device>,
  width: u32,
  height: u32,
  raw: Option<backend::Swapchain>,
  images: SmallVec<[(backend::Image, backend::ImageView); 3]>,
  framebuffers: SmallVec<[Arc<Framebuffer>; 3]>,
}

impl Swapchain {
  pub fn new(device: &Arc<Device>) -> Self {
    Swapchain {
      device: device.clone(),
      width: 0,
      height: 0,
      raw: None,
      images: SmallVec::new(),
      framebuffers: SmallVec::new(),
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

    for (i, image) in images.into_iter().enumerate() {
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
        .expect("could not create framebuffer");

      self.images.push((image, view));

      self.framebuffers.push(Arc::new(Framebuffer {
        index: i as u32,
        width,
        height,
        raw: framebuffer,
      }));
    }
  }

  pub fn raw_mut(&mut self) -> &mut backend::Swapchain {
    self.raw.as_mut().expect("swapchain is destroyed")
  }

  pub fn framebuffer(&self, index: u32) -> &Arc<Framebuffer> {
    &self.framebuffers[index as usize]
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

  pub fn acquire_framebuffer(
    &mut self,
    signal: &backend::Semaphore,
  ) -> Result<Arc<Framebuffer>, gfx_hal::AcquireError> {
    let index = self
      .raw_mut()
      .acquire_image(!0, gfx_hal::FrameSync::Semaphore(signal))?;

    Ok(self.framebuffers[index as usize].clone())
  }

  pub fn destroy(&mut self) {
    let device = &self.device.raw;

    for (image, view) in self.images.drain() {
      device.destroy_image_view(view);
      drop(image);
    }

    for framebuffer in self.framebuffers.drain() {
      if let Ok(framebuffer) = Arc::try_unwrap(framebuffer) {
        device.destroy_framebuffer(framebuffer.raw);
      } else {
        panic!("swapchain framebuffer still in use");
      }
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

pub struct Framebuffer {
  index: u32,
  raw: backend::Framebuffer,
  width: u32,
  height: u32,
}

impl Framebuffer {
  pub fn index(&self) -> u32 {
    self.index
  }

  pub fn raw(&self) -> &backend::Framebuffer {
    &self.raw
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }
}
