// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A device image.
pub struct Image {
  context: Arc<Context>,
  image: Expect<backend::Image>,
  view: Expect<backend::ImageView>,
  memory: Option<MemoryBlock>,
  size: Size<u32>,
}

impl Image {
  /// Creates a new device image of the given `size` in pixels.
  pub fn new(context: &Arc<Context>, size: Size<u32>) -> Result<Self, ImageCreationError> {
    let mut image = unsafe {
      context.device().create_image(
        gfx_hal::image::Kind::D2(size.width, size.height, 1, 1),
        1,
        gfx_hal::format::Format::Bgra8Unorm,
        gfx_hal::image::Tiling::Optimal,
        gfx_hal::image::Usage::TRANSFER_DST | gfx_hal::image::Usage::SAMPLED,
        gfx_hal::image::ViewCapabilities::empty(),
      )?
    };

    let requirements = unsafe { context.device().get_image_requirements(&image) };
    let memory = alloc(&context, requirements, MemoryProperties::DEVICE_LOCAL)?;

    unsafe {
      context.device().bind_image_memory(memory.as_backend(), 0, &mut image)?;
    }

    let view = unsafe {
      context.device().create_image_view(
        &image,
        gfx_hal::image::ViewKind::D2,
        gfx_hal::format::Format::Bgra8Unorm,
        gfx_hal::format::Swizzle::NO,
        gfx_hal::image::SubresourceRange {
          aspects: gfx_hal::format::Aspects::COLOR,
          levels: 0..1,
          layers: 0..1,
        },
      )?
    };

    Ok(Self {
      context: context.clone(),
      image: image.into(),
      view: view.into(),
      memory: memory.into(),
      size,
    })
  }

  /// Creates an [`Image`] to represent a swapchain image.
  pub fn from_swapchain_image(
    context: &Arc<Context>,
    image: backend::Image,
    size: Size<u32>,
    format: gfx_hal::format::Format,
  ) -> Result<Self, ImageCreationError> {
    let view = unsafe {
      context.device().create_image_view(
        &image,
        gfx_hal::image::ViewKind::D2,
        format,
        gfx_hal::format::Swizzle::NO,
        gfx_hal::image::SubresourceRange {
          aspects: gfx_hal::format::Aspects::COLOR,
          levels: 0..1,
          layers: 0..1,
        },
      )?
    };

    Ok(Self {
      context: context.clone(),
      image: image.into(),
      view: view.into(),
      memory: None,
      size,
    })
  }

  /// Returns the size of the image in pixels.
  pub fn size(&self) -> Size<u32> {
    self.size
  }

  /// Returns a reference to the underlying backend image.
  pub fn as_backend(&self) -> &backend::Image {
    &self.image
  }

  /// Returns a reference to the underlying backend image view.
  pub fn as_backend_view(&self) -> &backend::ImageView {
    &self.view
  }
}

// Implement `Drop` to destroy graphics resources.
impl Drop for Image {
  fn drop(&mut self) {
    let device = self.context.device();

    unsafe {
      device.destroy_image_view(self.view.take());

      // If memory is bound, then this is not a swapchain image so it must
      // be destroyed.
      if let Some(memory) = self.memory.take() {
        device.destroy_image(self.image.take());
        free(memory);
      }
    }
  }
}

/// An error that occurred during the creation of a new [`Image`].
#[derive(Debug)]
pub enum ImageCreationError {
  /// Out of either device or host memory to create resources with.
  OutOfMemory,
  /// Failed to allocate a memory block for the image.
  AllocationFailed(AllocationError),
}

impl std::error::Error for ImageCreationError {}

impl fmt::Display for ImageCreationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ImageCreationError::OutOfMemory => write!(f, "out of memory"),
      ImageCreationError::AllocationFailed(error) => {
        write!(f, "memory block allocation failed: {}", error)
      }
    }
  }
}

impl From<gfx_hal::image::CreationError> for ImageCreationError {
  fn from(error: gfx_hal::image::CreationError) -> Self {
    match error {
      gfx_hal::image::CreationError::OutOfMemory(_) => ImageCreationError::OutOfMemory,

      err => {
        panic!("failed to create image: {}", err);
      }
    }
  }
}

impl From<gfx_hal::image::ViewError> for ImageCreationError {
  fn from(error: gfx_hal::image::ViewError) -> Self {
    match error {
      gfx_hal::image::ViewError::OutOfMemory(_) => ImageCreationError::OutOfMemory,

      err => {
        panic!("failed to create image view: {}", err);
      }
    }
  }
}

impl From<AllocationError> for ImageCreationError {
  fn from(error: AllocationError) -> Self {
    ImageCreationError::AllocationFailed(error)
  }
}

impl From<gfx_hal::device::BindError> for ImageCreationError {
  fn from(error: gfx_hal::device::BindError) -> Self {
    match error {
      gfx_hal::device::BindError::OutOfMemory(_) => ImageCreationError::OutOfMemory,
      error => panic!("failed to bind image memory: {}", error),
    }
  }
}
