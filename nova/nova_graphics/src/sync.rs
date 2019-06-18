// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::backend;
use crate::{Context, OutOfMemoryError};
use gfx_hal::Device as _;
use std::sync::Arc;

/// Synchronization primitive used to control order of execution between command
/// buffers.
///
/// This structure is cloneable and all clones refer to the same semaphore. When
/// all clones are dropped, the underlying backend resource is destroyed.
#[derive(Clone)]
pub struct Semaphore(Arc<SemaphoreInner>);

struct SemaphoreInner {
  context: Context,
  semaphore: Option<backend::Semaphore>,
}

impl Semaphore {
  /// Creates a new semaphore in the given context.
  pub fn new(context: &Context) -> Result<Self, OutOfMemoryError> {
    let semaphore = context.device.create_semaphore()?;

    Ok(Self(Arc::new(SemaphoreInner {
      semaphore: Some(semaphore),
      context: context.clone(),
    })))
  }

  /// Returns a reference to the underlying backend semaphore.
  pub(crate) fn as_backend(&self) -> &backend::Semaphore {
    self.0.semaphore.as_ref().unwrap()
  }
}

impl Drop for SemaphoreInner {
  fn drop(&mut self) {
    unsafe {
      self
        .context
        .device
        .destroy_semaphore(self.semaphore.take().unwrap());
    }
  }
}

/// Synchronization primitive used to synchronize execution between the CPU and
/// the graphics device.
pub struct Fence {
  context: Context,
  fence: Option<backend::Fence>,
}

impl Fence {
  /// Creates a new fence in the given context.
  ///
  /// The `signaled` parameter sets the initial state of the fence.
  pub fn new(context: &Context, signaled: bool) -> Result<Self, OutOfMemoryError> {
    let fence = context.device.create_fence(signaled)?;

    Ok(Self {
      fence: Some(fence),
      context: context.clone(),
    })
  }

  /// Waits for the fence to be signaled, then resets it to unsignaled
  /// immediately.
  pub fn wait_and_reset(&self) {
    unsafe {
      self
        .context
        .device
        .wait_for_fence(self.as_backend(), !0)
        .expect("failed to wait for fence");

      self
        .context
        .device
        .reset_fence(self.as_backend())
        .expect("failed to reset fence");
    }
  }

  /// Returns a reference to the underlying backend fence.
  pub(crate) fn as_backend(&self) -> &backend::Fence {
    self.fence.as_ref().unwrap()
  }
}

impl Drop for Fence {
  fn drop(&mut self) {
    unsafe {
      self
        .context
        .device
        .destroy_fence(self.fence.take().unwrap())
    };
  }
}
