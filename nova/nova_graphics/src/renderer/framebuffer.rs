// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::backend;
use crate::renderer::RenderPass;
use crate::{Context, Image, OutOfMemoryError};
use gfx_hal::Device as _;
use nova_math::Size;
use std::cmp;
use std::iter;

pub struct Framebuffer {
  context: Context,
  render_pass: Option<RenderPass>,
  image: Option<Image>,
  framebuffer: Option<backend::Framebuffer>,
}

impl Framebuffer {
  pub fn new(context: &Context) -> Self {
    Self {
      context: context.clone(),
      render_pass: None,
      image: None,
      framebuffer: None,
    }
  }

  pub fn render_pass(&self) -> Option<&RenderPass> {
    self.render_pass.as_ref()
  }

  pub fn size(&self) -> Size<u32> {
    self.image.as_ref().map(Image::size).unwrap_or_default()
  }

  pub fn set_render_pass(&mut self, render_pass: &RenderPass) {
    match &self.render_pass {
      Some(value) if value == render_pass => return,
      _ => {
        self.render_pass = Some(render_pass.clone());
        self.destroy();
      }
    }
  }

  pub fn set_attachment(&mut self, image: &Image) {
    match &self.image {
      Some(value) if value == image => return,

      _ => {
        self.image = Some(image.clone());
        self.destroy();
      }
    }
  }

  pub(crate) fn ensure_created(&mut self) {
    if self.framebuffer.is_some() {
      return;
    }

    let image = self.image.as_ref().expect("an image is required");

    let render_pass = self
      .render_pass
      .as_ref()
      .expect("a render pass is required");

    let size = image.size();

    let mut extent = gfx_hal::image::Extent {
      depth: 1,
      width: size.width,
      height: size.height,
    };

    let framebuffer = unsafe {
      self
        .context
        .device
        .create_framebuffer(
          render_pass.as_backend(),
          iter::once(image.as_backend_view()),
          extent,
        )
        .expect("failed to create framebuffer")
    };

    self.framebuffer = Some(framebuffer);
  }

  pub(crate) fn as_backend(&self) -> &backend::Framebuffer {
    self
      .framebuffer
      .as_ref()
      .expect("framebuffer has not been created")
  }

  fn destroy(&mut self) {
    if let Some(framebuffer) = self.framebuffer.take() {
      unsafe {
        self
          .render_pass
          .as_ref()
          .unwrap()
          .context()
          .device
          .destroy_framebuffer(framebuffer);
      }
    }
  }
}

impl Drop for Framebuffer {
  fn drop(&mut self) {
    self.destroy();
  }
}
