// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::window::PresentMode;

use crate::graphics::backend;
use crate::graphics::hal::prelude::*;
use crate::graphics::image::{self, Image};
use crate::graphics::{Device, RenderPass, Semaphore};
use crate::math::Size;
use crate::utils::{quick_error, Droppable};
use smallvec::SmallVec;
use std::cmp;
use std::sync::Arc;

/// A set of backbuffer images that can be presented to a [`Surface`].
pub struct Swapchain {
  /// Reference to the device the swapchain was created with.
  device: Arc<Device>,
  /// Raw backend swapchain structure.
  raw: Droppable<backend::Swapchain>,
  /// Images in the swapchain.
  images: SmallVec<[Arc<Image>; 3]>, // No swapchain needs more than 3 images.
  /// Size of the swapchain in pixels.
  size: Size<u32>,
  /// Present mode of the swapchain.
  present_mode: PresentMode,
}

impl Swapchain {
  /// Creates a new swapchain compatible with the given render pass from a
  /// surface.
  ///
  /// The returned swapchain may not be the same size as requested.
  pub fn new(render_pass: &RenderPass, surface: &mut backend::Surface, size: Size<u32>) -> Self {
    let device = render_pass.device();

    // Determine surface capabilities and settings as well as available present
    // modes.
    let (caps, _, present_modes) = surface.compatibility(&device.adapter().physical_device);

    // Determine the best available extent of the swapchain.
    let extent = hal::window::Extent2D {
      width: cmp::max(
        caps.extents.start.width,
        cmp::min(size.width(), caps.extents.end.width),
      ),
      height: cmp::max(
        caps.extents.start.height,
        cmp::min(size.height(), caps.extents.end.height),
      ),
    };

    // Select the best available present mode and the image count needed for
    // that mode.
    let present_mode = select_present_mode(present_modes);

    let image_count = if present_mode == hal::window::PresentMode::Mailbox {
      // Mailbox should use three images if possible.
      cmp::min(caps.image_count.start, cmp::min(3, caps.image_count.end))
    } else {
      // Otherwise use the minimum number of images, which is always 2.
      caps.image_count.start
    };

    // Create a swapchain with the above config values.
    let config = hal::SwapchainConfig {
      present_mode,
      format: render_pass.format(),
      extent,
      image_count,
      image_layers: 1,
      image_usage: hal::image::Usage::COLOR_ATTACHMENT,
    };

    let (raw, backbuffer) = device
      .raw()
      .create_swapchain(surface, config, None)
      .expect("Could not create swapchain");

    let mut swapchain = Swapchain {
      device: device.clone(),
      raw: raw.into(),
      images: SmallVec::new(),
      size: extent.into(),
      present_mode,
    };

    // Extract the raw images from the enum result and create `Image` structs
    // for them.
    match backbuffer {
      hal::Backbuffer::Images(images) => {
        for image in images {
          swapchain.images.push(Arc::new(Image::from_raw(
            device,
            image::Backing::Swapchain(image),
            render_pass.format(),
            extent.into(),
          )));
        }
      }

      // I think this only happens with OpenGL, which isn't supported.
      _ => panic!("Device created framebuffer objects."),
    };

    swapchain
  }

  /// Gets a reference to the images in the swapchain.
  pub fn images(&self) -> &[Arc<Image>] {
    &self.images
  }

  /// Gets the size of the swapchain images in pixels.
  pub fn size(&self) -> Size<u32> {
    self.size
  }

  /// Gets the present mode of the swapchain.
  pub fn present_mode(&self) -> PresentMode {
    self.present_mode
  }

  /// Acquires an available image from the device for rendering. Returns the
  /// index of the image in the [`images()`] slice.
  ///
  /// The given semaphore will be signaled when the image is actually ready.
  pub fn acquire_image(&mut self, semaphore: &Semaphore) -> Result<usize, AcquireImageError> {
    let index = self
      .raw
      .acquire_image(!0, hal::FrameSync::Semaphore(semaphore.raw()))
      .map_err(|err| match err {
        hal::AcquireError::OutOfDate => AcquireImageError::OutOfDate,
        hal::AcquireError::NotReady => panic!("Swapchain::acquire_image timed out."),
        hal::AcquireError::SurfaceLost(_) => panic!("Surface lost."),
      })?;

    Ok(index as usize)
  }
}

// Implement `AsRef` to expose a reference to the raw backend swapchain.
impl AsRef<backend::Swapchain> for Swapchain {
  fn as_ref(&self) -> &backend::Swapchain {
    &self.raw
  }
}

// Implement `Drop` to destroy the swapchain.
impl Drop for Swapchain {
  fn drop(&mut self) {
    self.images.clear();

    if let Some(swapchain) = self.raw.take() {
      self.device.raw().destroy_swapchain(swapchain);
    }
  }
}

/// Selects the best available present mode from the given choices.
fn select_present_mode(modes: Vec<PresentMode>) -> PresentMode {
  // If mailbox is avaliable use it (for triple buffering).
  if modes.contains(&PresentMode::Mailbox) {
    PresentMode::Mailbox
  } else {
    // Fifo is always available.
    PresentMode::Fifo
  }
}

quick_error! {
  #[derive(Debug)]
  pub enum AcquireImageError {
    OutOfDate {
      display("The swapchain is out of date and must be recreated.")
    }
  }
}
