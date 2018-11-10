use super::backend;
use super::hal::prelude::*;
use super::{Device, Image, RenderPass};
use crate::math::algebra::Vector2;
use crate::utils::Droppable;
use std::sync::Arc;

pub struct Framebuffer {
  device: Arc<Device>,
  raw: Droppable<backend::Framebuffer>,
  images: Vec<Arc<Image>>,
  size: Vector2<u32>,
}

impl Framebuffer {
  pub fn new(render_pass: &Arc<RenderPass>, images: impl IntoIterator<Item = Arc<Image>>) -> Self {
    let device = render_pass.device();

    let images = images.into_iter().collect::<Vec<_>>();

    let extent = {
      let mut iter = images.iter();

      let size = iter
        .next()
        .expect("A framebuffer must have at least one image.")
        .size();

      if !iter.all(|img| img.size() == size) {
        panic!("All images in the framebuffer must be the same size.");
      }

      gfx_hal::image::Extent {
        width: size.x,
        height: size.y,
        depth: 1,
      }
    };

    let framebuffer = device
      .raw()
      .create_framebuffer(
        render_pass.raw(),
        images.iter().map(|img| img.as_ref().as_ref()),
        extent,
      )
      .expect("out of memory");

    Framebuffer {
      raw: framebuffer.into(),
      images,
      device: device.clone(),
      size: Vector2::new(extent.width, extent.height),
    }
  }

  pub fn size(&self) -> Vector2<u32> {
    self.size
  }
}

impl AsRef<backend::Framebuffer> for Framebuffer {
  fn as_ref(&self) -> &backend::Framebuffer {
    &self.raw
  }
}
