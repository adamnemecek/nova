// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Commands;
use crate::graphics::device::{self, Device, RawDeviceExt};
use crate::graphics::Backend;
use crate::utils::Droppable;
use std::sync::{Arc, RwLock};

use gfx_hal::pool::RawCommandPool as RawCommandPoolExt;

type RawCommandBuffer = <Backend as gfx_hal::Backend>::CommandBuffer;
type RawCommandPool = <Backend as gfx_hal::Backend>::CommandPool;

#[derive(Clone)]
pub struct CommandPool {
  inner: Arc<Inner>,
}

struct Inner {
  raw: Droppable<RwLock<RawCommandPool>>,
  device: Device,
  queue_id: device::QueueId,
}

impl CommandPool {
  pub fn new(device: &Device, queue_id: device::QueueId) -> Self {
    let raw = unsafe {
      device
        .raw()
        .create_command_pool(
          queue_id.family_id(),
          gfx_hal::pool::CommandPoolCreateFlags::TRANSIENT
            | gfx_hal::pool::CommandPoolCreateFlags::RESET_INDIVIDUAL,
        )
        .expect("Could not create command pool")
    };

    CommandPool {
      inner: Arc::new(Inner {
        device: device.clone(),
        raw: RwLock::new(raw).into(),
        queue_id,
      }),
    }
  }

  pub fn queue_id(&self) -> device::QueueId {
    self.inner.queue_id
  }

  pub fn acquire(&self) -> Commands {
    let raw_buffer = self
      .inner
      .raw
      .write()
      .unwrap()
      .allocate_one(gfx_hal::command::RawLevel::Primary);

    Commands::new(raw_buffer, self)
  }

  pub(super) fn release_raw(&self, raw_buffer: RawCommandBuffer) {
    unsafe {
      self.inner.raw.write().unwrap().free(Some(raw_buffer));
    }
  }
}

impl Drop for Inner {
  fn drop(&mut self) {
    if let Some(raw) = self.raw.take() {
      unsafe {
        self
          .device
          .raw()
          .destroy_command_pool(raw.into_inner().unwrap());
      }
    }
  }
}
