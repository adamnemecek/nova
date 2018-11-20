// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{AcquireImageError, Surface, Swapchain};
use crate::graphics::device::{self, DeviceHandle};
use crate::graphics::image::Image;
use crate::graphics::prelude::*;
use crate::graphics::sync::Semaphore;
use crate::utils::Droppable;
use crate::window::Window;
use std::iter;
use std::sync::Arc;

pub struct Presenter {
  surface: Surface,
  queue: device::QueueHandle,
  semaphores: Vec<Arc<Semaphore>>,
  swapchain: Droppable<Swapchain>,
}

impl Presenter {
  pub fn new(device: &DeviceHandle, window: &Window) -> Presenter {
    let surface = Surface::new(device, window);
    let queue = device::get_present_queue(&surface);

    let semaphores = iter::repeat_with(|| Arc::new(Semaphore::new(device)))
      .take(3)
      .collect();

    Presenter {
      surface,
      queue,
      semaphores,
      swapchain: Droppable::dropped(),
    }
  }

  pub fn begin(&mut self) -> Backbuffer {
    for _ in 0..1 {
      if self.swapchain.is_dropped() {
        self.swapchain = Swapchain::new(&mut self.surface).into();
      }

      let result = self
        .swapchain
        .acquire_image(self.semaphores.last().unwrap());

      match result {
        Ok(index) => {
          let semaphore = self.semaphores.pop().unwrap();

          self.semaphores.insert(0, semaphore);

          return Backbuffer {
            presenter: self,
            image: index,
          };
        }

        Err(AcquireImageError::OutOfDate) => {
          self.swapchain = Droppable::dropped();
        }
      };
    }

    panic!("Swapchain was repeatedly out of date.");
  }

  fn finish<'a>(&mut self, image: usize, wait_for: impl IntoIterator<Item = &'a Arc<Semaphore>>) {
    debug_assert!(image < self.swapchain.images().len());

    let result = self.queue.lock().raw_mut().present(
      iter::once((self.swapchain.as_ref(), image as u32)),
      wait_for.into_iter().map(AsRef::as_ref).map(AsRef::as_ref),
    );

    if result.is_err() {
      self.swapchain = Droppable::dropped();
    }
  }
}

pub struct Backbuffer<'a> {
  presenter: &'a mut Presenter,
  image: usize,
}

impl<'a> Backbuffer<'a> {
  pub fn index(&self) -> usize {
    self.image
  }

  pub fn image(&self) -> &Arc<Image> {
    &self.presenter.swapchain.images()[self.image]
  }

  pub fn semaphore(&self) -> &Arc<Semaphore> {
    self.presenter.semaphores.first().unwrap()
  }

  pub fn present<'b>(self, wait_for: impl IntoIterator<Item = &'b Arc<Semaphore>>) {
    self.presenter.finish(self.image, wait_for)
  }
}
