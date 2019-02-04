// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Pass;
use crate::graphics::{Backend, Device, Image, RawDeviceExt};
use crate::math::Size;
use crate::utils::Droppable;
use std::sync::{Arc, Weak};

type RawFramebuffer = <Backend as gfx_hal::Backend>::Framebuffer;

pub struct Framebuffer {
  raw: Droppable<RawFramebuffer>,
  device: Device,
  size: Size<u32>,
}

impl Framebuffer {
  pub fn new(pass: &Pass, size: Size<u32>, attachments: Vec<Arc<Image>>) -> Self {
    let device = pass.device().clone();

    let raw = unsafe {
      device
        .raw()
        .create_framebuffer(
          pass.raw(),
          attachments.iter().map(|a| a.raw_view()),
          size.into(),
        )
        .expect("Could not create framebuffer")
    };

    Framebuffer {
      device,
      raw: raw.into(),
      size,
    }
  }

  pub fn size(&self) -> Size<u32> {
    self.size
  }

  pub(crate) fn raw(&self) -> &RawFramebuffer {
    &self.raw
  }
}

impl Drop for Framebuffer {
  fn drop(&mut self) {
    if let Some(raw) = self.raw.take() {
      unsafe {
        self.device.raw().destroy_framebuffer(raw);
      }
    }
  }
}

pub struct CachedFramebuffer {
  pass: Pass,
  attachments: Vec<Weak<Image>>,
  size: Option<Size<u32>>,
  cache: Option<Framebuffer>,
}

impl CachedFramebuffer {
  pub fn new(pass: &Pass) -> Self {
    CachedFramebuffer {
      pass: pass.clone(),
      attachments: vec![Weak::new(); pass.attachment_count()],
      size: None,
      cache: None,
    }
  }

  pub fn attach(&mut self, index: usize, image: &Weak<Image>) {
    let current = &mut self.attachments[index];

    match (image.upgrade(), current.upgrade()) {
      (None, None) => return,
      (Some(ref image), Some(ref current)) if Arc::ptr_eq(current, image) => return,

      _ => {
        *current = image.clone();
        self.cache = None;
      }
    }
  }

  pub fn set_size(&mut self, size: Size<u32>) {
    match self.size {
      Some(current) if current == size => return,

      _ => {
        self.size = Some(size);
        self.cache = None;
      }
    }
  }

  pub fn will_create(&mut self) -> bool {
    self.cache.is_none()
  }

  pub fn get_or_create(&mut self) -> &Framebuffer {
    match self.cache {
      Some(ref fb) => fb,

      None => {
        let size = self
          .size
          .expect("Cannot create a framebuffer with no size.");

        let attachments = self
          .attachments
          .iter()
          .cloned()
          .map(|a| {
            a.upgrade()
              .expect("Not all framebuffer attachments have been set.")
          })
          .collect();

        self.cache = Some(Framebuffer::new(&self.pass, size, attachments));
        self.cache.as_ref().unwrap()
      }
    }
  }
}
