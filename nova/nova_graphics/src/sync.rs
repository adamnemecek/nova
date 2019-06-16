// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::backend;
use crate::Context;
use gfx_hal::Device as _;

pub struct Semaphore {
  context: Context,
  semaphore: Option<backend::Semaphore>,
}

impl Semaphore {
  pub fn new(context: &Context) -> Self {
    Self {
      semaphore: context
        .device
        .create_semaphore()
        .expect("failed to create semaphore")
        .into(),
      context: context.clone(),
    }
  }

  pub(crate) fn as_backend(&self) -> &backend::Semaphore {
    self.semaphore.as_ref().unwrap()
  }
}

impl Drop for Semaphore {
  fn drop(&mut self) {
    unsafe {
      self
        .context
        .device
        .destroy_semaphore(self.semaphore.take().unwrap())
    };
  }
}

pub struct Fence {
  context: Context,
  fence: Option<backend::Fence>,
}

impl Fence {
  pub fn new(context: &Context) -> Self {
    Self {
      fence: context
        .device
        .create_fence(true)
        .expect("failed to create fence")
        .into(),
      context: context.clone(),
    }
  }

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
