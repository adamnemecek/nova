// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::device::{Device, DeviceExt};
use super::Backend;

pub type Semaphore = <Backend as gfx_hal::Backend>::Semaphore;
pub type Fence = <Backend as gfx_hal::Backend>::Fence;

pub struct FrameSync {
  pub texture_semaphore: Semaphore,
  pub backbuffer_semaphore: Semaphore,
  pub render_semaphore: Semaphore,
  pub fence: Fence,
}

impl FrameSync {
  pub fn new(device: &Device) -> FrameSync {
    let texture_semaphore = device.create_semaphore().unwrap();
    let backbuffer_semaphore = device.create_semaphore().unwrap();
    let render_semaphore = device.create_semaphore().unwrap();
    let fence = device.create_fence(true).unwrap();

    FrameSync {
      texture_semaphore,
      backbuffer_semaphore,
      render_semaphore,
      fence,
    }
  }

  pub fn wait_for_fence(&self, device: &Device) {
    unsafe {
      device
        .wait_for_fence(&self.fence, !0)
        .expect("Could not wait for fence");

      device
        .reset_fence(&self.fence)
        .expect("Could not reset fence");
    }
  }

  pub fn destroy(self, device: &Device) {
    unsafe {
      device.destroy_semaphore(self.backbuffer_semaphore);
      device.destroy_semaphore(self.render_semaphore);
      device.destroy_fence(self.fence);
    }
  }
}
