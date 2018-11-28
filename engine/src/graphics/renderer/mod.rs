// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod descriptor;
pub mod pipeline;
pub mod shader;
pub mod vertex;

mod pass;
mod render;
pub mod surface;

pub use self::render::Render;
pub use self::surface::Surface;

use super::commands::{CommandLevel, CommandPool, Commands};
use super::device;
use super::prelude::*;
use crate::math::Size;
use std::sync::{Arc, Weak};

pub struct Renderer {
  render_pass: backend::RenderPass,
  command_pool: CommandPool,
  framebuffers: Vec<Option<Framebuffer>>,
}

impl Renderer {
  pub fn new(device: &device::Handle) -> Self {
    let render_pass = pass::create_render_pass(device);
    let command_pool = CommandPool::new(device, device::get_graphics_queue_family_id(device));

    Renderer {
      render_pass,
      command_pool,
      framebuffers: Vec::new(),
    }
  }

  pub fn render_frame(&mut self, backbuffer: &Weak<surface::Backbuffer>) -> Render {
    // Ensure there are at least as many framebuffer slots as there are
    // backbuffers.
    while self.framebuffers.len() <= backbuffer.index() {
      self.framebuffers.push(None);
    }

    // Get the framebuffer corresponding to the backbuffer index.
    let framebuffer = &mut self.framebuffers[backbuffer.index()];

    // If the framebuffer hasn't been created or is out of date, recreate it.
    let out_of_date = framebuffer
      .as_mut()
      .map(|fb| !Weak::ptr_eq(fb.backbuffer, backbuffer))
      .unwrap_or(true);

    if out_of_date {
      let raw = self.device.raw().create_framebuffer();

      *framebuffer = Some(Framebuffer {
        backbuffer: backbuffer.clone(),
        raw,
      });
    }

    let commands = Commands::new(&self.command_pool, CommandLevel::Primary);

    let render = Render::new(
      &mut commands,
      &self.render_pass,
      framebuffer.as_ref().unwrap(),
    );

    commands.finish();
  }
}

struct Framebuffer {
  raw: backend::Framebuffer,
  backbuffer: Weak<surface::Backbuffer>,
  index: usize,
  size: Size<i16>,
}

impl Framebuffer {
  pub fn size(&self) -> Size<i16> {
    self.size
  }
}
