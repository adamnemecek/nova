// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::backend;
use crate::Context;
use gfx_hal::Device as _;
use nova_log as log;
use nova_math::Size;
use std::cmp;
use std::sync::Arc;

/// An image on the graphics device.
#[derive(Clone)]
pub struct Image(Arc<ImageInner>);

struct ImageInner {
  context: Context,
  image: Option<backend::Image>,
  view: Option<backend::ImageView>,
  size: Size<u32>,
}

impl Image {
  pub(crate) fn from_swapchain_image(
    context: &Context,
    image: backend::Image,
    size: Size<u32>,
    format: gfx_hal::format::Format,
  ) -> Self {
    let view = unsafe {
      context
        .device
        .create_image_view(
          &image,
          gfx_hal::image::ViewKind::D2,
          format,
          gfx_hal::format::Swizzle::NO,
          gfx_hal::image::SubresourceRange {
            aspects: gfx_hal::format::Aspects::COLOR,
            levels: 0..1,
            layers: 0..1,
          },
        )
        .expect("Could not create image view")
    };

    Self(Arc::new(ImageInner {
      context: context.clone(),
      image: Some(image),
      view: Some(view),
      size,
    }))
  }

  pub fn size(&self) -> Size<u32> {
    self.0.size
  }

  pub(crate) fn as_backend_view(&self) -> &backend::ImageView {
    self.0.view.as_ref().unwrap()
  }
}

impl Drop for ImageInner {
  fn drop(&mut self) {
    unsafe {
      self
        .context
        .device
        .destroy_image_view(self.view.take().unwrap());

      /*
      self
        .context
        .device
        .destroy_image(self.image.take().unwrap());
        */
    }
  }
}

impl cmp::PartialEq for Image {
  fn eq(&self, other: &Image) -> bool {
    Arc::ptr_eq(&self.0, &other.0)
  }
}
