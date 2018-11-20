// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Framebuffer, RenderPass};
use crate::graphics::commands::Commands;
use crate::graphics::present::Backbuffer;
use std::sync::Arc;

/// Executes a render pass with framebuffers lazily created from attached
/// backbuffers.
pub struct Renderer {
  /// Render pass to execute.
  pass: Arc<RenderPass>,
  /// List of framebuffers, one for each backbuffer index. If `None`, the
  /// framebuffer has not yet been created.
  framebuffers: Vec<Option<Framebuffer>>,
  /// Index of the last framebuffer returned from `begin()`. This is the index
  /// of the most recently attached backbuffer.
  frame: usize,
}

impl Renderer {
  /// Creates a new renderer for executing the given render pass.
  pub fn new(pass: &Arc<RenderPass>) -> Self {
    Renderer {
      pass: pass.clone(),
      framebuffers: Vec::new(),
      frame: 0,
    }
  }

  /// Gets a reference to the render pass this renderer executes.
  pub fn pass(&self) -> &Arc<RenderPass> {
    &self.pass
  }

  /// Attaches the given backbuffer to the renderer's framebuffer.
  pub fn attach(&mut self, backbuffer: &Backbuffer) {
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
      .map(|fb| !Arc::ptr_eq(&fb.images()[0], backbuffer.image()))
      .unwrap_or(true);

    if out_of_date {
      *framebuffer = Some(Framebuffer::new(
        &self.pass,
        vec![backbuffer.image().clone()],
      ));
    }

    // Make sure the next render uses this framebuffer.
    self.frame = backbuffer.index();
  }

  /// Begins the render pass for the given set of commands and returns a
  /// reference to the framebuffer used.
  pub fn begin(&mut self, commands: &mut Commands) -> &Framebuffer {
    let framebuffer = self
      .framebuffers
      .get(self.frame)
      .and_then(Option::as_ref)
      .expect(
        "No framebuffer has been created. Attach an image with `attach()` to create a framebuffer.",
      );

    commands.begin_render_pass(&self.pass, framebuffer);

    framebuffer
  }

  /// Finishes the render pass for the given set of commands.
  pub fn finish(&mut self, commands: &mut Commands) {
    commands.finish_render_pass();
  }
}
