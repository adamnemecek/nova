// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::window::PresentMode;

use super::Surface;
use crate::graphics::backend;
use crate::graphics::hal::prelude::*;
use crate::graphics::image::{self, Image};
use crate::graphics::{Device, Semaphore};
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
}

impl Swapchain {
  /// Creates a new swapchain with the given device.
  ///
  /// The returned swapchain may not be the same size as requested.
  pub fn new(device: &Arc<Device>, surface: &mut Surface, size: Size<u32>) -> Self {
    assert!(
      Arc::ptr_eq(device.backend(), surface.backend()),
      "Device and surface were created with different backend instances."
    );

    let surface: &mut backend::Surface = surface.as_mut();
    let (caps, _, _) = surface.compatibility(&device.adapter().physical_device);

    let format = image::Format::Bgra8Unorm;

    let extent = match caps.current_extent {
      Some(e) => e,        // Use the actual size of the surface.
      None => size.into(), // Any size is allowed. Use the given size.
    };

    let image_count = match caps.image_count.end {
      0 => 2, // Any number of images is allowed. Only need two.
      x => cmp::min(x, 2),
    };

    let config = hal::SwapchainConfig {
      present_mode: hal::window::PresentMode::Fifo,
      format,
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
    };

    // Extract the raw images from the enum result and create `Image` structs
    // for them.
    match backbuffer {
      hal::Backbuffer::Images(images) => {
        for image in images {
          swapchain.images.push(Arc::new(Image::from_raw(
            device,
            image::Backing::Swapchain(image),
            image::Format::Bgra8Unorm,
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

  /// Acquires an available image from the device for rendering. Returns the
  /// index of the image in the [`images()`] slice.
  ///
  /// The given semaphore will be signaled when the image is actually ready.
  pub fn acquire_image(&mut self, semaphore: &Semaphore) -> Result<usize, AcquireImageError> {
    let index = self
      .raw
      .acquire_image(!0, hal::FrameSync::Semaphore(semaphore.as_ref()))
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
    // Wait for all queues to be empty so the swapchain is definitely not in
    // use.
    self.device.wait_idle();

    self.images.clear();

    if let Some(swapchain) = self.raw.take() {
      self.device.raw().destroy_swapchain(swapchain);
    }
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
