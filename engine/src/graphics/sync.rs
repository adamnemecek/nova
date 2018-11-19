// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Device;
use crate::graphics::prelude::*;
use crate::utils::Droppable;
use std::sync::Arc;

/// A synchronization primitive for inserting dependencies into command
/// execution.
///
/// Semaphores can be passed to some asynchronous operations such as submitting
/// comands or acquiring a semaphore which will signal the semaphore on
/// completion. Other operations accept a list of semaphores to wait on before
/// executing.
pub struct Semaphore {
  device: Arc<Device>,
  raw: Droppable<backend::Semaphore>,
}

impl Semaphore {
  /// Creates a new semaphore with the given device.
  pub fn new(device: &Arc<Device>) -> Self {
    let semaphore = device
      .raw()
      .create_semaphore()
      .expect("could not create semaphore");

    Semaphore {
      raw: semaphore.into(),
      device: device.clone(),
    }
  }
}

// Implement `AsRef` to expose the raw backend semaphore.
impl AsRef<backend::Semaphore> for Semaphore {
  fn as_ref(&self) -> &backend::Semaphore {
    &self.raw
  }
}

// Implement `Drop` to destroy the raw backend semaphore.
impl Drop for Semaphore {
  fn drop(&mut self) {
    if let Some(semaphore) = self.raw.take() {
      self.device.raw().destroy_semaphore(semaphore);
    }
  }
}

/// A synchronization primitive that the device signals on completion of some
/// operation.
pub struct Fence {
  /// Device the fence was created for.
  device: Arc<Device>,
  /// Raw backend fence structure.
  raw: Droppable<backend::Fence>,
}

impl Fence {
  /// Creates a new fence for the given device.
  pub fn new(device: &Arc<Device>) -> Self {
    let fence = device
      .raw()
      .create_fence(true) // Initially signaled.
      .expect("Could not create fence");

    Fence {
      raw: fence.into(),
      device: device.clone(),
    }
  }

  /// Checks if the fence is currently signaled.
  pub fn is_signaled(&self) -> bool {
    self
      .device
      .raw()
      .get_fence_status(&self.raw)
      .unwrap_or(false)
  }

  /// Waits for the fence to be signaled.
  pub fn wait(&self) {
    self
      .device
      .raw()
      .wait_for_fence(&self.raw, !0)
      .expect("Could not wait for fence");
  }

  /// Resets the fence to unsignaled.
  pub fn reset(&mut self) {
    self
      .device
      .raw()
      .reset_fence(&self.raw)
      .expect("Could not reset fence");
  }

  /// Waits for the fence to be signaled then resets it to unsignaled.
  pub fn wait_and_reset(&mut self) {
    self.wait();
    self.reset();
  }
}

// Implement `AsRef` to expose the raw backend fence structure.
impl AsRef<backend::Fence> for Fence {
  fn as_ref(&self) -> &backend::Fence {
    &self.raw
  }
}

// Implement `Drop` to destroy the fence on the device.
impl Drop for Fence {
  fn drop(&mut self) {
    if let Some(fence) = self.raw.take() {
      self.device.raw().destroy_fence(fence);
    }
  }
}
