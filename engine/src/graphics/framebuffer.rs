// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Backend, Device, Image, RawDeviceExt, RenderPass};
use crate::math::Size;
use crate::utils::Droppable;
use std::sync::{Arc, Weak};

type RawFramebuffer = <Backend as gfx_hal::Backend>::Framebuffer;

pub struct Framebuffer {
  raw: Droppable<RawFramebuffer>,
  device: Device,
}

impl Framebuffer {
  pub fn new(render_pass: &RenderPass, attachments: Vec<Arc<Image>>, size: Size<u32>) -> Self {
    let device = render_pass.device().clone();

    let raw = unsafe {
      device
        .raw()
        .create_framebuffer(
          render_pass.raw(),
          attachments.iter().map(|a| a.raw_view()),
          size.into(),
        )
        .expect("Could not create framebuffer")
    };

    Framebuffer {
      device,
      raw: raw.into(),
    }
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
  render_pass: RenderPass,
  attachments: Vec<Weak<Image>>,
  size: Option<Size<u32>>,
  cache: Option<Framebuffer>,
}

impl CachedFramebuffer {
  pub fn new(render_pass: &RenderPass) -> Self {
    CachedFramebuffer {
      render_pass: render_pass.clone(),
      attachments: vec![Weak::new(); render_pass.attachment_count()],
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

  pub fn get(&mut self) -> &Framebuffer {
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

        self.cache = Some(Framebuffer::new(&self.render_pass, attachments, size));

        self.cache.as_ref().unwrap()
      }
    }
  }
}

pub struct FramebufferCache {
  buffers: Vec<Option<CachedFramebuffer>>,
  render_pass: RenderPass,
}

impl FramebufferCache {
  pub fn new(render_pass: &RenderPass) -> Self {
    FramebufferCache {
      buffers: Vec::new(),
      render_pass: render_pass.clone(),
    }
  }

  pub fn cached(
    &mut self,
    index: usize,
    edit: impl FnOnce(&mut CachedFramebuffer),
  ) -> &Framebuffer {
    while self.buffers.len() <= index {
      self.buffers.push(None);
    }

    let buffer = &mut self.buffers[index];

    let cached = match buffer {
      Some(v) => v,

      None => {
        *buffer = Some(CachedFramebuffer::new(&self.render_pass));
        buffer.as_mut().unwrap()
      }
    };

    edit(cached);

    cached.get()
  }
}
