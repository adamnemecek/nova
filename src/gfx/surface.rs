// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use gfx_hal::{Surface as _, Swapchain as _};

/// A rendering surface.
pub struct Surface {
  window: ArcWeak<winit::Window>,
  context: Arc<Context>,
  surface: backend::Surface,

  present_queue_id: cmd::QueueId,
  size: Size<f64>,
  resized: bool,

  swapchain: Option<backend::Swapchain>,
  swapchain_images: Vec<Arc<Image>>,

  logger: log::Logger,
}

impl Surface {
  /// Format of all surfaces.
  pub const FORMAT: gfx_hal::format::Format = gfx_hal::format::Format::Bgra8Unorm;

  /// Creates a new surface using the given window.
  pub fn new(context: &Arc<Context>, window: &window::Handle, logger: &log::Logger) -> Self {
    let surface = context.backend().create_surface(window.as_winit());
    let present_queue_id = context.queues().find_present_queue(&surface);

    Self {
      window: Arc::downgrade(window.as_winit()),
      context: context.clone(),
      surface,

      present_queue_id,
      size: Size::default(),
      resized: false,

      swapchain: None,
      swapchain_images: Vec::new(),

      logger: logger.clone(),
    }
  }

  /// Returns a reference to the graphics context this surface was created in.
  pub fn context(&self) -> &Arc<Context> {
    &self.context
  }

  /// Acquire a backbuffer from the render surface.
  ///
  /// If the given `signal` semaphore is provided, it will be signaled when the
  /// backbuffer is ready for use.
  pub fn acquire<'a>(
    &'a mut self,
    signal: impl Into<Option<&'a cmd::Semaphore>>,
  ) -> Result<Backbuffer, SurfaceAcquireError> {
    // Get the current window size in pixels.
    let window = self.window.upgrade().ok_or(SurfaceAcquireError::WindowClosed)?;

    let size = window
      .get_inner_size()
      .ok_or(SurfaceAcquireError::WindowClosed)?
      .to_physical(window.get_hidpi_factor());

    let size = Size::new(size.width, size.height);

    // Destroy the swapchain if its size has changed.
    if self.size != size {
      self.destroy_swapchain();
      self.size = size;
    }

    // Ensure the swapchain has been created.
    if self.swapchain.is_none() {
      self.create_swapchain();
    }

    // Acquire an image from the surface.
    let signal = signal.into().map(cmd::Semaphore::as_backend);

    let index = loop {
      let image = unsafe { self.swapchain.as_mut().unwrap().acquire_image(!0, signal, None) };

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

    Ok(Backbuffer { surface: self, index, presented: false })
  }

  /// Creates the underlying swapchain.
  fn create_swapchain(&mut self) {
    let (capabilities, _, _) = self.surface.compatibility(self.context.physical_device());

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
        .device()
        .create_swapchain(&mut self.surface, config, None)
        .expect("Could not create swapchain")
    };

    self.swapchain = Some(swapchain);

    let size = Size::new(extent.width, extent.height);

    self.size = Size::new(f64::from(size.width), f64::from(size.height));

    log::debug!(&self.logger, "created swapchain";
      "image_count" => image_count,
      "format" => log::Debug(Self::FORMAT),
      "size" => log::Debug(size),
    );

    for image in backbuffers {
      self.swapchain_images.push(
        Image::from_swapchain_image(&self.context, image, size, Self::FORMAT)
          .expect("failed to create swapchain image")
          .into(),
      );
    }
  }

  /// Destroys the underlying swapchain.
  fn destroy_swapchain(&mut self) {
    self.swapchain_images.clear();

    if let Some(swapchain) = self.swapchain.take() {
      unsafe { self.context.device().destroy_swapchain(swapchain) };
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
  pub fn image(&self) -> &Arc<Image> {
    &self.surface.swapchain_images[self.index as usize]
  }

  /// Presents the backbuffer to the render surface.
  ///
  /// Presentation will wait until all of the given semaphores, if any, are
  /// signaled.
  pub fn present(mut self, wait_semaphores: &[&cmd::Semaphore]) -> Result<(), SurfacePresentError> {
    debug_assert!(!self.presented, "already presented");

    let swapchain = self.surface.swapchain.as_ref().unwrap();

    let result = self.surface.context.queues().present(
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
      log::error!(&self.surface.logger, "backbuffer was neverü presented");
    }
  }
}

/// An error that occurred while acquiring a backbuffer from a render surface.
#[derive(Debug, PartialEq, Eq)]
pub enum SurfaceAcquireError {
  /// The device is out of memory.
  OutOfMemory,
  /// The window has been closed.
  WindowClosed,
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
        SurfaceAcquireError::WindowClosed => "window closed",
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
#[derive(Debug, PartialEq, Eq)]
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
