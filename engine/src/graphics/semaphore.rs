// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Device;
use crate::graphics::backend;
use crate::graphics::hal::prelude::*;
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
