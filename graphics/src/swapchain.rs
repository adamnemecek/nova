use super::{RenderImage, RenderTarget};
use gfx_hal::pool::RawCommandPool;
use gfx_hal::{Device, Surface};
use std::cmp;

pub const MAX_IMAGE_COUNT: usize = 3;

pub fn create(target: &mut RenderTarget, width: u32, height: u32) {
  let device = &target.context.device;
  let surface = &mut target.surface;
  let log = &mut target.log;

  let compatibility = surface.compatibility(&target.context.adapter.physical_device);
  let config = configure(target.format, width, height, compatibility);
  let extent = config.extent;
  let (swapchain, backbuffer) = device.create_swapchain(surface, config, None);

  target.swapchain = Some(swapchain);

  let images = match backbuffer {
    gfx_hal::Backbuffer::Images(images) => images,
    _ => panic!("device created framebuffer objects"),
  };

  for image in images {
    let view = device
      .create_image_view(
        &image,
        gfx_hal::image::ViewKind::D2,
        target.format,
        gfx_hal::format::Swizzle::NO,
        gfx_hal::image::SubresourceRange {
          aspects: gfx_hal::format::Aspects::COLOR,
          levels: 0..1,
          layers: 0..1,
        },
      )
      .expect("could not create image view");

    let framebuffer = device
      .create_framebuffer(target.render_pass.raw(), Some(&view), extent.to_extent())
      .unwrap();

    target.images.push(RenderImage {
      _raw: image,
      view,
      framebuffer,
    });
  }

  log
    .trace("Created swapchain.")
    .with("width", &width)
    .with("height", &height);
}

pub fn destroy(target: &mut RenderTarget) {
  let device = &target.context.device;
  let log = &mut target.log;

  for image in target.images.drain() {
    device.destroy_framebuffer(image.framebuffer);
    device.destroy_image_view(image.view);
  }

  if let Some(swapchain) = target.swapchain.take() {
    device.destroy_swapchain(swapchain);

    log.trace("Destroyed swapchain.");
  }
}

fn configure(
  format: gfx_hal::format::Format,
  width: u32,
  height: u32,
  compatibility: (
    gfx_hal::window::SurfaceCapabilities,
    Option<Vec<gfx_hal::format::Format>>,
    Vec<gfx_hal::window::PresentMode>,
  ),
) -> gfx_hal::SwapchainConfig {
  let (caps, _, modes) = compatibility;

  let image_count = cmp::max(
    caps.image_count.start,
    cmp::min(MAX_IMAGE_COUNT as u32, caps.image_count.end),
  );

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

  gfx_hal::SwapchainConfig {
    present_mode: select_present_mode(modes),
    format,
    extent,
    image_count,
    image_layers: 1,
    image_usage: gfx_hal::image::Usage::COLOR_ATTACHMENT,
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
