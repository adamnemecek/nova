// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use gfx_hal::{Surface as _, Swapchain as _};

// Rendering surface created from a window.
pub struct Surface {
  size: Size<f64>,
  surface: backend::Surface,
  present_queue_id: cmd::QueueId,
  resized: bool,

  swapchain: Option<backend::Swapchain>,
  swapchain_images: Vec<Image>,

  context: Context,
}

impl Surface {
  /// Format of all surfaces.
  pub(crate) const FORMAT: gfx_hal::format::Format = gfx_hal::format::Format::Bgra8Unorm;

  /// Creates a new render surface using the given window.
  pub fn new(context: &Context, window: &window::Handle) -> Self {
    let size = window.size();
    let surface = context.backend.create_surface(window.as_ref());
    let present_queue_id = context.queues.find_present_queue(&surface);

    Self {
      size,
      surface,
      present_queue_id,
      resized: false,

      swapchain: None,
      swapchain_images: Vec::new(),

      context: context.clone(),
    }
  }

  /// Returns the size of the surface in pixels.
  pub fn size(&self) -> Size<f64> {
    self.size
  }

  /// Sets the size of the render surface.
  ///
  /// Call this function if the window size changes.
  pub fn resize(&mut self, size: Size<f64>) {
    if size != self.size {
      self.size = size;
      self.resized = true;
    }
  }

  /// Acquire a backbuffer from the render surface.
  ///
  /// If the given `signal` semaphore is provided, it will be signaled when the
  /// backbuffer is ready for use.
  pub fn acquire<'a>(
    &'a mut self,
    signal: impl Into<Option<&'a Semaphore>>,
  ) -> Result<Backbuffer, SurfaceAcquireError> {
    if self.resized {
      self.resized = false;
      self.destroy_swapchain();
    }

    if self.swapchain.is_none() {
      self.create_swapchain();
    }

    let signal = signal.into().map(Semaphore::as_backend);

    let index = loop {
      let image = unsafe {
        self
          .swapchain
          .as_mut()
          .unwrap()
          .acquire_image(!0, signal, None)
      };

      match image {
        Ok((index, None)) => {
          break index;
        }

        Err(err) if err != gfx_hal::AcquireError::OutOfDate => {
          return Err(err.into());
        }

        _ => {}
      }
    };

    Ok(Backbuffer {
      surface: self,
      index,
      presented: false,
    })
  }

  /// Creates the underlying swapchain.
  fn create_swapchain(&mut self) {
    let (capabilities, _, _) = self
      .surface
      .compatibility(&self.context.adapter.physical_device);

    let extent = gfx_hal::window::Extent2D {
      width: math::clamp(
        self.size.width.round() as u32,
        capabilities.extents.start.width..capabilities.extents.end.width,
      ),
      height: math::clamp(
        self.size.height.round() as u32,
        capabilities.extents.start.height..capabilities.extents.end.height,
      ),
    };

    let image_count = match capabilities.image_count.end {
      0 => 2, // Any number of images is allowed. Only need two.
      x => cmp::min(x, 2),
    };

    let config = gfx_hal::SwapchainConfig {
      present_mode: gfx_hal::window::PresentMode::Fifo,
      format: Self::FORMAT,
      extent,
      image_count,
      image_layers: 1,
      image_usage: gfx_hal::image::Usage::COLOR_ATTACHMENT,
      composite_alpha: gfx_hal::window::CompositeAlpha::OPAQUE,
    };

    let (swapchain, backbuffers) = unsafe {
      self
        .context
        .device
        .create_swapchain(&mut self.surface, config, None)
        .expect("Could not create swapchain")
    };

    self.swapchain = Some(swapchain);

    let size = Size::new(extent.width, extent.height);

    self.size = Size::new(f64::from(size.width), f64::from(size.height));

    log::debug!(self.context.logger(),
      "created swapchain";
      "image_count" => image_count,
      "format" => log::Debug(Self::FORMAT),
      "size" => log::Debug(size),
    );

    for image in backbuffers {
      self.swapchain_images.push(
        Image::from_swapchain_image(&self.context, image, size, Self::FORMAT)
          .expect("failed to create swapchain image"),
      );
    }
  }

  /// Destroys the underlying swapchain.
  fn destroy_swapchain(&mut self) {
    self.swapchain_images.clear();

    if let Some(swapchain) = self.swapchain.take() {
      unsafe { self.context.device.destroy_swapchain(swapchain) };
    }
  }
}

impl Drop for Surface {
  fn drop(&mut self) {
    self.destroy_swapchain();
  }
}

/// An image acquired from a `Surface` for rendering.
pub struct Backbuffer<'a> {
  surface: &'a mut Surface,
  index: u32,
  presented: bool,
}

impl<'a> Backbuffer<'a> {
  /// Returns a reference to the `Image` representing the backbuffer.
  pub fn image(&self) -> &Image {
    &self.surface.swapchain_images[self.index as usize]
  }

  /// Presents the backbuffer to the render surface.
  ///
  /// Presentation will wait until all of the given semaphores, if any, are
  /// signaled.
  pub fn present(mut self, wait_semaphores: &[&Semaphore]) -> Result<(), SurfacePresentError> {
    debug_assert!(!self.presented, "already presented");

    let swapchain = self.surface.swapchain.as_ref().unwrap();

    let result = self.surface.context.queues.present(
      self.surface.present_queue_id,
      swapchain,
      self.index,
      wait_semaphores,
    );

    self.presented = true;

    match result {
      Ok(()) => Ok(()),

      Err(gfx_hal::window::PresentError::OutOfDate) => {
        self.surface.destroy_swapchain();

        Ok(())
      }

      Err(err) => Err(err.into()),
    }
  }
}

impl<'a> Drop for Backbuffer<'a> {
  fn drop(&mut self) {
    if !self.presented {
      log::error!(
        self.surface.context.logger(),
        "backbuffer was not presented"
      );
    }
  }
}

/// An error that occurred while acquiring a backbuffer from a render surface.
#[derive(Debug)]
pub enum SurfaceAcquireError {
  /// The device is out of memory.
  OutOfMemory,
  /// The surface is no longer usable.
  SurfaceLost,
  /// The device is no longer usable.
  DeviceLost,
}

impl fmt::Display for SurfaceAcquireError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        SurfaceAcquireError::OutOfMemory => "out of memory",
        SurfaceAcquireError::SurfaceLost => "surface lost",
        SurfaceAcquireError::DeviceLost => "device lost",
      }
    )
  }
}

impl From<gfx_hal::AcquireError> for SurfaceAcquireError {
  fn from(value: gfx_hal::AcquireError) -> Self {
    match value {
      gfx_hal::AcquireError::OutOfMemory(_) => SurfaceAcquireError::OutOfMemory,
      gfx_hal::AcquireError::SurfaceLost(_) => SurfaceAcquireError::SurfaceLost,
      gfx_hal::AcquireError::DeviceLost(_) => SurfaceAcquireError::DeviceLost,

      gfx_hal::AcquireError::NotReady => {
        panic!("surface acquire timeout expired but should be infinite");
      }

      gfx_hal::AcquireError::OutOfDate => {
        panic!("out of date surface should be handled automatically");
      }
    }
  }
}

/// An error that occurred while presenting a backbuffer from a render surface.
#[derive(Debug)]
pub enum SurfacePresentError {
  /// The device is out of memory.
  OutOfMemory,
  /// The surface is no longer usable.
  SurfaceLost,
  /// The device is no longer usable.
  DeviceLost,
}

impl fmt::Display for SurfacePresentError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        SurfacePresentError::OutOfMemory => "out of memory",
        SurfacePresentError::SurfaceLost => "surface lost",
        SurfacePresentError::DeviceLost => "device lost",
      }
    )
  }
}

impl From<gfx_hal::window::PresentError> for SurfacePresentError {
  fn from(value: gfx_hal::window::PresentError) -> Self {
    match value {
      gfx_hal::window::PresentError::OutOfMemory(_) => SurfacePresentError::OutOfMemory,
      gfx_hal::window::PresentError::SurfaceLost(_) => SurfacePresentError::SurfaceLost,
      gfx_hal::window::PresentError::DeviceLost(_) => SurfacePresentError::DeviceLost,

      gfx_hal::window::PresentError::OutOfDate => {
        panic!("out of date surface should be handled automatically");
      }
    }
  }
}
