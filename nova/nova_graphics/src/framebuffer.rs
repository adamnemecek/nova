// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Container for the resources needed to render a single frame.
///
/// The underlying backend framebuffer will not be created until a render pass
/// and a backbuffer attachment are set. Until then, trying to use the
/// framebuffer will cause a panic.
///
/// If either the render pass or backbuffer attachment change, the framebuffer
/// will be recreated automatically. It is the caller's responsibility to ensure
/// that the framebuffer is no longer in use at that time.
pub struct Framebuffer {
  context: Context,
  render_pass: Option<RenderPass>,
  image: Option<Image>,
  framebuffer: Option<backend::Framebuffer>,
}

impl Framebuffer {
  /// Creates a new framebuffer with no render pass or attachment.
  pub fn new(context: &Context) -> Self {
    Self {
      context: context.clone(),
      render_pass: None,
      image: None,
      framebuffer: None,
    }
  }

  /// Returns the size of the framebuffer.
  pub fn size(&self) -> Size<u32> {
    self.image.as_ref().map(Image::size).unwrap_or_default()
  }

  /// Returns the render pass assigned to the framebuffer, if any.
  pub fn render_pass(&self) -> Option<&RenderPass> {
    self.render_pass.as_ref()
  }

  /// Sets the render pass the framebuffer will be used for.
  pub fn set_render_pass(&mut self, render_pass: &RenderPass) {
    match &self.render_pass {
      Some(value) if value == render_pass => return,
      _ => {
        self.render_pass = Some(render_pass.clone());
        self.destroy();
      }
    }
  }

  /// Sets the backbuffer image attachment.
  pub fn set_attachment(&mut self, image: &Image) {
    match &self.image {
      Some(value) if value == image => return,

      _ => {
        self.image = Some(image.clone());
        self.destroy();
      }
    }
  }

  /// Ensures that the framebuffer is created.
  ///
  /// Panics if this is not possible.
  pub fn ensure_created(&mut self) -> Result<(), OutOfMemoryError> {
    if self.framebuffer.is_some() {
      return Ok(());
    }

    let render_pass = self
      .render_pass
      .as_ref()
      .expect("framebuffer requires a render pass");

    let image = self
      .image
      .as_ref()
      .expect("framebuffer requires a backbuffer attachment");

    let size = image.size();

    let extent = gfx_hal::image::Extent {
      depth: 1,
      width: size.width,
      height: size.height,
    };

    let framebuffer = unsafe {
      self.context.device.create_framebuffer(
        render_pass.as_backend(),
        iter::once(image.as_backend_view()),
        extent,
      )?
    };

    self.framebuffer = Some(framebuffer);

    Ok(())
  }

  /// Returns a reference to the underlying backend framebuffer.
  ///
  /// Panics if the framebuffer has not been created.
  pub(crate) fn as_backend(&self) -> &backend::Framebuffer {
    self
      .framebuffer
      .as_ref()
      .expect("framebuffer has not been created")
  }

  /// Destroys the underlying backend framebuffer.
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
