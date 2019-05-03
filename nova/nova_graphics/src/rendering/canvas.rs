// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::gpu::{CommandBuffer, Gpu};
use crate::rendering::pipeline::{Pipeline, PipelineOptions};
use crate::rendering::shader::{Shader, ShaderCode, ShaderKind};
use crate::rendering::{Framebuffer, RenderPass};

pub struct Canvas {
  render_pass: RenderPass,
  pipeline: Pipeline,
  vertex_shader: Shader,
  fragment_shader: Shader,
  commands: CommandBuffer,
}

impl Canvas {
  pub fn new(gpu: &Gpu, commands: CommandBuffer) -> Self {
    let render_pass = RenderPass::new(gpu);

    let vertex_shader = {
      let code = ShaderCode::compile(ShaderKind::Vertex, include_str!("shaders/quad.vert"))
        .expect("Could not compile vertex shader");

      Shader::new(gpu, &code)
    };

    let fragment_shader = {
      let code = ShaderCode::compile(ShaderKind::Fragment, include_str!("shaders/color.frag"))
        .expect("Could not compile fragment shader");

      Shader::new(gpu, &code)
    };

    let pipeline = Pipeline::new(
      gpu,
      &render_pass,
      PipelineOptions {
        vertex_shader: &vertex_shader,
        fragment_shader: &fragment_shader,
        size_of_push_constants: 0,
      },
    );

    Self {
      render_pass,
      pipeline,
      vertex_shader,
      fragment_shader,
      commands,
    }
  }

  pub(crate) fn commands(&self) -> &CommandBuffer {
    &self.commands
  }

  pub fn begin(&mut self, framebuffer: &Framebuffer) {
    self.commands.begin();

    self
      .commands
      .begin_render_pass(&self.render_pass, framebuffer);

    self.commands.bind_pipeline(&self.pipeline);
  }

  pub fn draw_quad(&mut self) {
    self.commands.draw(0..4);
  }

  pub fn finish(&mut self) {
    self.commands.finish_render_pass();
    self.commands.finish();
  }

  pub fn destroy(self, gpu: &Gpu) {
    self.commands.destroy(gpu);
    self.pipeline.destroy(gpu);
    self.render_pass.destroy(gpu);
    self.vertex_shader.destroy(gpu);
    self.fragment_shader.destroy(gpu);
  }
}
