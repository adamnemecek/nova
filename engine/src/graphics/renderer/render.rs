// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Framebuffer, Renderer};
use crate::graphics::commands::{CommandPool, Commands};
use crate::graphics::prelude::*;

pub struct Render<'a> {
  commands: &'a mut Commands,
}

impl Render<'a> {
  pub(super) fn new(
    commands: &'a mut Commands,
    render_pass: &'a backend::RenderPass,
    framebuffer: &'a Framebuffer,
  ) -> Self {
    commands.begin_render_pass(render_pass, framebuffer);

    Render { commands }
  }

  pub(super) fn finish(self) {
    self.commands.finish_render_pass();
  }
}
