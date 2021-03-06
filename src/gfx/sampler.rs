// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

pub struct Sampler {
  context: Arc<Context>,
  sampler: Expect<backend::Sampler>,
}

impl Sampler {
  pub fn new(context: &Arc<Context>, filter: SamplerFilter) -> Result<Self, SamplerCreationError> {
    let sampler = unsafe {
      context.device().create_sampler(gfx_hal::image::SamplerInfo::new(
        match filter {
          SamplerFilter::Nearest => gfx_hal::image::Filter::Nearest,
          SamplerFilter::Linear => gfx_hal::image::Filter::Linear,
        },
        gfx_hal::image::WrapMode::Tile,
      ))?
    };

    Ok(Self { context: context.clone(), sampler: sampler.into() })
  }

  pub fn as_backend(&self) -> &backend::Sampler {
    &self.sampler
  }
}

impl Drop for Sampler {
  fn drop(&mut self) {
    unsafe {
      self.context.device().destroy_sampler(self.sampler.take());
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum SamplerFilter {
  Nearest,
  Linear,
}

/// An error that occurred while creating a `Sampler`.
#[derive(Debug, Clone, Copy)]
pub enum SamplerCreationError {
  /// Out of either host or device memory.
  OutOfMemory,
  /// Too many samplers have been allocated.
  TooManyBlocks,
}

impl std::error::Error for SamplerCreationError {}

impl From<gfx_hal::device::AllocationError> for SamplerCreationError {
  fn from(error: gfx_hal::device::AllocationError) -> Self {
    match error {
      gfx_hal::device::AllocationError::OutOfMemory(_) => SamplerCreationError::OutOfMemory,
      gfx_hal::device::AllocationError::TooManyObjects => SamplerCreationError::TooManyBlocks,
    }
  }
}

impl fmt::Display for SamplerCreationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      SamplerCreationError::OutOfMemory => write!(f, "out of memory"),
      SamplerCreationError::TooManyBlocks => write!(f, "too many samplers allocated"),
    }
  }
}
