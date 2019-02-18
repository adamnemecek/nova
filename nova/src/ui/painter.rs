// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Color, Layout, Style};
use crate::ecs::{self, Join};
use crate::engine;
use crate::math::Matrix4;
use crate::renderer::{self, Renderer};
use crate::window::Window;

pub struct Painter {
  pipeline: renderer::Pipeline,
}

impl Painter {
  pub fn new(renderer: &Renderer) -> Self {
    let vertex_shader = renderer::Shader::new(
      renderer.device(),
      &renderer::shader::Spirv::from_glsl(
        renderer::ShaderKind::Vertex,
        include_str!("shaders/panels.vert"),
      ),
    );

    let fragment_shader = renderer::Shader::new(
      renderer.device(),
      &renderer::shader::Spirv::from_glsl(
        renderer::ShaderKind::Fragment,
        include_str!("shaders/panels.frag"),
      ),
    );

    let pipeline = renderer::PipelineBuilder::new()
      .set_vertex_shader(&vertex_shader)
      .set_fragment_shader(&fragment_shader)
      .add_push_constant::<Matrix4<f32>>()
      .add_push_constant::<[f32; 4]>()
      .add_push_constant::<Color>()
      .build(renderer.device(), renderer.render_pass())
      .expect("Could not create graphics pipeline");

    Painter { pipeline }
  }

  pub fn draw(&mut self, mut cmd: renderer::DrawCommands, res: &engine::Resources) {
    // Scale the entire UI based on the size of the window.
    let size = res.fetch::<Window>().size();

    let scale = if size.height() > size.width() {
      (size.width() / 1280).max(1) as f32
    } else {
      (size.height() / 720).max(1) as f32
    };

    // Create a projection matrix that converts UI units to screen space.
    let projection = Matrix4::new_orthographic(
      0.0,
      size.width() as f32,
      0.0,
      size.height() as f32,
      -1.0,
      1.0,
    )
    .prepend_scaling(scale);

    cmd.bind_pipeline(&self.pipeline);
    cmd.push_constant(&self.pipeline, 0, &projection);

    let layouts = ecs::read_components::<Layout>(res);
    let styles = ecs::read_components::<Style>(res);

    for (layout, style) in (&layouts, &styles).join() {
      if style.background.a <= 0.0 {
        continue;
      }

      cmd.push_constant(
        &self.pipeline,
        1,
        &[layout.x, layout.y, layout.width, layout.height],
      );

      cmd.push_constant(&self.pipeline, 2, &style.background);

      cmd.draw(0..4);
    }
  }

  pub fn destroy(self, device: &renderer::Device) {
    self.pipeline.destroy(device);
  }
}