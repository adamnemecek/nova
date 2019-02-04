// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Backend, Device, RawDeviceExt};
use crate::utils::Droppable;

type RawSemaphore = <Backend as gfx_hal::Backend>::Semaphore;
type RawFence = <Backend as gfx_hal::Backend>::Fence;

pub struct Semaphore {
  raw: Droppable<RawSemaphore>,
  device: Device,
}

impl Semaphore {
  pub fn new(device: &Device) -> Semaphore {
    let raw = device
      .raw()
      .create_semaphore()
      .expect("Could not create semaphore");

    Semaphore {
      device: device.clone(),
      raw: raw.into(),
    }
  }

  pub(crate) fn raw(&self) -> &RawSemaphore {
    &self.raw
  }
}

impl Drop for Semaphore {
  fn drop(&mut self) {
    if let Some(raw) = self.raw.take() {
      unsafe {
        self.device.raw().destroy_semaphore(raw);
      }
    }
  }
}

pub struct Fence {
  raw: Droppable<RawFence>,
  device: Device,
}

impl Fence {
  pub fn new(device: &Device) -> Fence {
    let raw = device
      .raw()
      .create_fence(true)
      .expect("Could not create fence");

    Fence {
      device: device.clone(),
      raw: raw.into(),
    }
  }

  pub(crate) fn raw(&self) -> &RawFence {
    &self.raw
  }

  /// Waits for the fence to be signaled.
  pub fn wait(&self) {
    unsafe {
      self
        .device
        .raw()
        .wait_for_fence(&self.raw, !0)
        .expect("Could not wait for fence");
    }
  }

  /// Resets the fence to unsignaled.
  pub fn reset(&mut self) {
    unsafe {
      self
        .device
        .raw()
        .reset_fence(&self.raw)
        .expect("Could not reset fence");
    }
  }

  /// Waits for the fence to be signaled then resets it to unsignaled.
  pub fn wait_and_reset(&mut self) {
    self.wait();
    self.reset();
  }
}

impl Drop for Fence {
  fn drop(&mut self) {
    if let Some(raw) = self.raw.take() {
      unsafe {
        self.device.raw().destroy_fence(raw);
      }
    }
  }
}
