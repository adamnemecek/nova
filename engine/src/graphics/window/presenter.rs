// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::swapchain::{AcquireImageError, Swapchain};
use super::{Surface, Window};
use crate::graphics::commands::CommandQueue;
use crate::graphics::prelude::*;
use crate::graphics::{Device, Image, Semaphore};
use crate::math::Size;
use crate::utils::Droppable;
use std::iter;
use std::sync::Arc;

pub struct Presenter {
  device: Arc<Device>,
  surface: Surface,
  size: Size<u32>,
  swapchain: Droppable<Swapchain>,
  semaphores: Vec<Arc<Semaphore>>,
}

impl Presenter {
  pub fn new(device: &Arc<Device>, window: &mut Window) -> Presenter {
    let surface = window.take_surface();

    assert!(
      Arc::ptr_eq(device.backend(), surface.backend()),
      "Device and window must be created with the same backend instance."
    );

    let semaphores = iter::repeat_with(|| Arc::new(Semaphore::new(&device)))
      .take(3)
      .collect();

    Presenter {
      device: device.clone(),
      surface,
      size: window.size(),
      swapchain: Droppable::dropped(),
      semaphores,
    }
  }

  pub fn begin(&mut self) -> Backbuffer {
    for _ in 0..1 {
      if self.swapchain.is_dropped() {
        self.swapchain = Swapchain::new(&self.device, &mut self.surface, self.size).into();
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

  fn finish<'a>(
    &mut self,
    image: usize,
    queue: &mut CommandQueue,
    wait_for: impl IntoIterator<Item = &'a Arc<Semaphore>>,
  ) {
    debug_assert!(image < self.swapchain.images().len());

    let queue: &mut backend::CommandQueue = queue.raw_mut();

    let result = queue.present(
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

  pub fn present<'b>(
    self,
    queue: &mut CommandQueue,
    wait_for: impl IntoIterator<Item = &'b Arc<Semaphore>>,
  ) {
    self.presenter.finish(self.image, queue, wait_for)
  }
}
