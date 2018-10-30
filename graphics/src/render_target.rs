use super::backend;
use super::{Backend, RenderPass};
use gfx_hal::pool::{CommandPoolCreateFlags, RawCommandPool};
use gfx_hal::{Device, Surface};
use smallvec::SmallVec;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
use std::sync::Arc;

pub struct RenderTarget {
  render_pass: Arc<RenderPass>,
  swapchain: Option<backend::Swapchain>,
  extent: gfx_hal::window::Extent2D,
  images: SmallVec<[RefCell<RenderImage>; 3]>,
  log: bflog::Logger,
}

pub struct RenderImage {
  _raw: backend::Image,
  view: backend::ImageView,
  framebuffer: backend::Framebuffer,
}

impl RenderTarget {
  pub fn new(
    render_pass: &Arc<RenderPass>,
    width: u32,
    height: u32,
    image_count: usize,
  ) -> Rc<RenderTarget> {
    let context = render_pass.context();
    let device = context.device();

    let mut log = context.log().with_src("graphics::Swapchain");

    let mut surface = context
      .surface()
      .lock()
      .expect("could not lock context surface");

    let (surface_caps, _, present_modes) =
      surface.compatibility(&context.adapter().physical_device);

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

    let swapchain_config = gfx_hal::SwapchainConfig {
      present_mode: select_present_mode(present_modes),
      format: context.format(),
      extent,
      image_count: cmp::min(image_count as u32, surface_caps.image_count.end),
      image_layers: 1,
      image_usage: gfx_hal::image::Usage::COLOR_ATTACHMENT,
    };

    let (swapchain, backbuffer) = device.create_swapchain(&mut surface, swapchain_config, None);

    let images = match backbuffer {
      gfx_hal::Backbuffer::Images(images) => images,
      _ => panic!("device created framebuffer objects"),
    };

    let image_count = images.len();

    let images = images
      .into_iter()
      .map(|image| {
        let view = device
          .create_image_view(
            &image,
            gfx_hal::image::ViewKind::D2,
            context.format(),
            gfx_hal::format::Swizzle::NO,
            gfx_hal::image::SubresourceRange {
              aspects: gfx_hal::format::Aspects::COLOR,
              levels: 0..1,
              layers: 0..1,
            },
          ).expect("could not create image view");

        let framebuffer = device
          .create_framebuffer(render_pass.pass(), Some(view), extent.to_extent())
          .unwrap();

        RenderImage {
          _raw: image,
          view,
          framebuffer,
        }
      }).map(RefCell::new)
      .collect();

    let mut command_pool = device.create_command_pool(
      context.graphics_queue_family,
      CommandPoolCreateFlags::TRANSIENT | CommandPoolCreateFlags::RESET_INDIVIDUAL,
    );

    let command_buffers = command_pool.allocate(image_count, gfx_hal::command::RawLevel::Primary);

    log
      .trace("Created.")
      .with("width", &extent.width)
      .with("height", &extent.height)
      .with("image_count", &image_count);

    Rc::new(RenderTarget {
      render_pass: render_pass.clone(),
      swapchain: Some(swapchain),
      extent,
      images,
      log,
    })
  }
}

impl Drop for RenderTarget {
  fn drop(&mut self) {
    let device = self.render_pass.context().device();

    while let Some(image) = self.images.pop() {
      let image = image.into_inner();

      device.destroy_framebuffer(image.framebuffer);
      device.destroy_image_view(image.view);
    }

    if let Some(swapchain) = self.swapchain.take() {
      device.destroy_swapchain(swapchain);
    }

    self.log.trace("Destroyed.");
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
