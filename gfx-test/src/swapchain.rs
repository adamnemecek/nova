use super::gfx_back;
use super::gfx_back::{Backend, Framebuffer, Image, ImageView, RenderPass, Semaphore};
use super::Context;
use gfx_hal;
use gfx_hal::format::{Aspects, ChannelType, Format, Swizzle};
use gfx_hal::image::{Extent, SubresourceRange, ViewKind};
use gfx_hal::window::{PresentMode, SurfaceCapabilities};
use gfx_hal::{Backbuffer, CommandQueue, Device, FrameSync, Graphics, Surface, Swapchain};

pub use gfx_hal::SwapchainConfig;

const COLOR_RANGE: SubresourceRange = SubresourceRange {
  aspects: Aspects::COLOR,
  levels: 0..1,
  layers: 0..1,
};

pub struct SwapchainContext {
  pub swapchain: gfx_back::Swapchain,
  pub extent: Extent,
  pub frame_views: Vec<(Image, ImageView)>,
  pub framebuffers: Vec<Framebuffer>,
}

pub fn create(
  device: &gfx_back::Device,
  surface: &mut gfx_back::Surface,
  surface_caps: &SurfaceCapabilities,
  surface_format: Format,
  render_pass: &RenderPass,
) -> SwapchainContext {
  // Create a swapchain config from the caps and selected color format and
  // store its extent.
  //
  // TODO: Can I control the size? Is it based on the window size?
  let mut config = gfx_hal::SwapchainConfig::from_caps(&surface_caps, surface_format);
  let extent = config.extent.to_extent();

  // If there's space, add one extra image to the swapchain config for
  // triple-buffering.
  //
  // TODO: Is this needed? Should I only do this if the mode is mailbox?
  if surface_caps.image_count.end > config.image_count {
    config.image_count += 1;
  }

  // Create a swapchain and backbuffer from the swapchain config.
  let (swapchain, backbuffer) = device.create_swapchain(surface, config, None);

  let (frame_views, framebuffers) = match backbuffer {
    Backbuffer::Images(images) => {
      let pairs = images
        .into_iter()
        .map(|image| {
          let rtv = device
            .create_image_view(
              &image,
              ViewKind::D2,
              surface_format,
              Swizzle::NO,
              COLOR_RANGE.clone(),
            ).unwrap();
          (image, rtv)
        }).collect::<Vec<_>>();
      let fbos = pairs
        .iter()
        .map(|&(_, ref rtv)| {
          device
            .create_framebuffer(render_pass, Some(rtv), extent)
            .unwrap()
        }).collect();

      (pairs, fbos)
    }

    Backbuffer::Framebuffer(fbo) => (Vec::new(), vec![fbo]),
  };

  SwapchainContext {
    swapchain,
    extent,
    frame_views,
    framebuffers,
  }
}

pub fn acquire_frame(ctx: &mut SwapchainContext, semaphore: &Semaphore) -> u32 {
  ctx
    .swapchain
    .acquire_image(!0 /* no timeout */, FrameSync::Semaphore(semaphore))
    .expect("could not acquire frame")
}

pub fn present(
  ctx: &mut SwapchainContext,
  queue: &mut CommandQueue<Backend, Graphics>,
  image_index: u32,
) {
  ctx.swapchain.present(queue, image_index, &[]);
}

pub fn destroy(device: &gfx_back::Device, ctx: SwapchainContext) {
  for framebuffer in ctx.framebuffers {
    device.destroy_framebuffer(framebuffer);
  }

  for (_image, image_view) in ctx.frame_views {
    // Swapchain images are destroyed with the swapchain, so just destroy views.
    device.destroy_image_view(image_view);
  }

  device.destroy_swapchain(ctx.swapchain);
}
