// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::backend;
use crate::{Context, OutOfMemoryError, QueueId};
use crossbeam_queue::SegQueue;
use gfx_hal::pool::RawCommandPool as _;
use gfx_hal::Device as _;
use std::iter;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct Pool(Arc<PoolInner>);

struct PoolInner {
  context: Context,
  pool: Option<Mutex<backend::CommandPool>>,
  level: gfx_hal::command::RawLevel,
  recycled_buffers: SegQueue<backend::CommandBuffer>,
  recording: AtomicBool,
}

impl Pool {
  pub fn new(context: &Context, queue_id: QueueId) -> Result<Self, OutOfMemoryError> {
    let pool = unsafe {
      context.device.create_command_pool(
        queue_id.family_id,
        gfx_hal::pool::CommandPoolCreateFlags::RESET_INDIVIDUAL,
      )
    };

    match pool {
      Ok(pool) => Ok(Pool(Arc::new(PoolInner {
        pool: Some(Mutex::new(pool)),
        context: context.clone(),
        level: gfx_hal::command::RawLevel::Primary,
        recycled_buffers: SegQueue::new(),
        recording: AtomicBool::new(false),
      }))),

      Err(_) => Err(OutOfMemoryError),
    }
  }

  pub(crate) fn allocate(&self) -> backend::CommandBuffer {
    self
      .0
      .recycled_buffers
      .pop()
      .unwrap_or_else(|_| self.as_backend().allocate_one(self.0.level))
  }

  pub(crate) fn recycle(&self, buffer: backend::CommandBuffer) {
    self.0.recycled_buffers.push(buffer);
  }

  pub(crate) fn is_recording(&self) -> &AtomicBool {
    &self.0.recording
  }

  pub(crate) fn as_backend(&self) -> MutexGuard<backend::CommandPool> {
    self.0.pool.as_ref().unwrap().lock().unwrap()
  }
}

impl Drop for PoolInner {
  fn drop(&mut self) {
    let mut pool = self.pool.take().unwrap().into_inner().unwrap();

    while let Ok(buffer) = self.recycled_buffers.pop() {
      unsafe {
        pool.free(iter::once(buffer));
      }
    }

    unsafe {
      self.context.device.destroy_command_pool(pool);
    }
  }
}
