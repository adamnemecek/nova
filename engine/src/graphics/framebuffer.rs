// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::backend;
use super::hal::prelude::*;
use super::{Device, Image, RenderPass};
use crate::math::Size;
use crate::utils::Droppable;
use std::sync::Arc;

/// A set of images used by a render pass.
pub struct Framebuffer {
  /// Raw backend framebuffer structure.
  raw: Droppable<backend::Framebuffer>,
  /// Size of the framebuffer in pixels.
  size: Size<u32>,
  /// Images in the framebuffer. Stored to prevent them from being dropped.
  _images: Vec<Arc<Image>>,
  /// Device the framebuffer was created with. Stored to prevent it from being
  /// dropped.
  _device: Arc<Device>,
}

impl Framebuffer {
  /// Creates a new framebuffer from a set of images that is compatible with the
  /// given render pass.
  pub fn new(render_pass: &Arc<RenderPass>, images: impl IntoIterator<Item = Arc<Image>>) -> Self {
    let device = render_pass.device();

    // Collect the images to store them.
    let images = images.into_iter().collect::<Vec<_>>();

    // Get the extent (size) of the images, panicing if any differ.
    let extent = {
      let mut iter = images.iter();

      let size = iter
        .next()
        .expect("A framebuffer must have at least one image.")
        .size();

      if !iter.all(|img| img.size() == size) {
        panic!("All images in a framebuffer must be of the same size.");
      }

      size.into()
    };

    // Create the framebuffer.
    let image_views = images
      .iter()
      .map(AsRef::as_ref) // Arc<Image> -> &Image
      .map(AsRef::as_ref); // &Image -> &backend::ImageView

    let framebuffer = device
      .raw()
      .create_framebuffer(render_pass.raw(), image_views, extent)
      .expect("Out of memory.");

    Framebuffer {
      raw: framebuffer.into(),
      size: extent.into(),
      _images: images,
      _device: device.clone(),
    }
  }

  /// Gets the size of the framebuffer in pixels.
  pub fn size(&self) -> Size<u32> {
    self.size
  }
}

// Implement `AsRef` to expose the raw backend framebuffer.
impl AsRef<backend::Framebuffer> for Framebuffer {
  fn as_ref(&self) -> &backend::Framebuffer {
    &self.raw
  }
}
