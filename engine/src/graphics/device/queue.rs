// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::queue::RawSubmission;

use super::Device;
use crate::graphics::backend;
use crate::graphics::hal::prelude::*;
use crate::graphics::window::Swapchain;
use crate::graphics::Semaphore;
use std::sync::Arc;

/// A device queue for submitting [`Commands`] or presenting [`Swapchain`]
/// images.
pub struct Queue {
  /// Raw backend queue family information.
  family: backend::QueueFamily,
  /// Raw backend queue structure.
  raw: backend::CommandQueue,
  /// Device the queue was created with.
  device: Arc<Device>,
}

impl Queue {
  /// Creates a new queue from the given raw backend structures.
  ///
  /// Unsafe because this function does not verify that the given queues belong
  /// to the device.
  pub unsafe fn from_raw(
    device: &Arc<Device>,
    queues: &mut hal::queue::Queues,
    family: backend::QueueFamily,
  ) -> Self {
    let raw = queues
      .take_raw(family.id())
      .expect("Expected device queue family was missing.")
      .into_iter()
      .next()
      .expect("Expected device queue was missing.");

    Queue {
      family,
      raw,
      device: device.clone(),
    }
  }

  /// Gets a reference to the device the queue was created with.
  pub fn device(&self) -> &Arc<Device> {
    &self.device
  }

  /// Gets the ID of the queue family the queue belongs to.
  pub fn family_id(&self) -> usize {
    self.family.id().0
  }

  /// Presents swapchain images. Presentation will wait for all of the given
  /// semaphores.
  ///
  /// Swapchain images are specified with a tuple containing a reference to the
  /// swapchain and the index of the image to present.
  pub fn present<'a>(
    &mut self,
    images: impl IntoIterator<Item = (&'a Swapchain, u32)>,
    wait_for: impl IntoIterator<Item = &'a Semaphore>,
  ) -> Result<(), ()> {
    self.raw.present(
      images.into_iter().map(|(sc, i)| (sc.as_ref(), i)),
      wait_for.into_iter().map(Semaphore::raw),
    )
  }

  /// Gets a mutable reference to the raw backend queue.
  pub fn raw_mut(&mut self) -> &mut backend::CommandQueue {
    &mut self.raw
  }
}
