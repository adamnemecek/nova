use super::{RenderFrame, RenderTarget};
use gfx_hal::pool::RawCommandPool;
use gfx_hal::{Device, Surface};
use std::cmp;

pub const MAX_IMAGE_COUNT: usize = 3;

pub fn create(target: &mut RenderTarget, width: u32, height: u32) {
  let device = &target.context.device;
  let surface = &mut target.surface;
  let log = &mut target.log;

  let mut command_buffers = target
    .command_pool
    .as_mut()
    .expect("command pool was destroyed")
    .allocate(MAX_IMAGE_COUNT, gfx_hal::command::RawLevel::Primary);

  let (surface_caps, _, present_modes) =
    surface.compatibility(&target.context.adapter.physical_device);

  let extent = gfx_hal::window::Extent2D {
    width: cmp::max(
      surface_caps.extents.start.width,
      cmp::min(width, surface_caps.extents.end.width),
    ),
    height: cmp::max(
      surface_caps.extents.start.height,
      cmp::min(height, surface_caps.extents.end.height),
    ),
  };

  let image_count = cmp::max(
    surface_caps.image_count.start,
    cmp::min(MAX_IMAGE_COUNT as u32, surface_caps.image_count.end),
  );

  let swapchain_config = gfx_hal::SwapchainConfig {
    present_mode: select_present_mode(present_modes),
    format: target.format,
    extent,
    image_count,
    image_layers: 1,
    image_usage: gfx_hal::image::Usage::COLOR_ATTACHMENT,
  };

  let (swapchain, backbuffer) = device.create_swapchain(surface, swapchain_config, None);

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
      ).expect("could not create image view");

    let buffer = device
      .create_framebuffer(target.render_pass.raw(), Some(&view), extent.to_extent())
      .unwrap();

    target.frames.push(RenderFrame {
      _image: image,
      view,
      buffer,
      command_buffer: command_buffers.pop().unwrap(),
      fence: device.create_fence(true),
      acquire_semaphore: device.create_semaphore(),
      render_semaphore: device.create_semaphore(),
    });
  }

  log
    .trace("Created swapchain.")
    .with("width", &width)
    .with("height", &height);
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
