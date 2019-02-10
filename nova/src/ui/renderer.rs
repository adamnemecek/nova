// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Background, Layout};
use crate::ecs;
use crate::engine;
use crate::graphics;
use crate::math::Matrix4;
use crate::window::Window;

pub struct Renderer {
  pipeline: graphics::Pipeline,
}

impl Renderer {
  pub fn new(pass: &graphics::renderer::Pass) -> Self {
    let vertex_shader = graphics::pipeline::Shader::new(
      pass.device(),
      &graphics::pipeline::shader::Spirv::from_glsl(
        graphics::pipeline::ShaderKind::Vertex,
        include_str!("shaders/panels.vert"),
      ),
    );

    let fragment_shader = graphics::pipeline::Shader::new(
      pass.device(),
      &graphics::pipeline::shader::Spirv::from_glsl(
        graphics::pipeline::ShaderKind::Fragment,
        include_str!("shaders/panels.frag"),
      ),
    );

    let pipeline = graphics::PipelineBuilder::new()
      .set_render_pass(&pass)
      .set_vertex_shader(&vertex_shader)
      .set_fragment_shader(&fragment_shader)
      .add_push_constant::<Matrix4<f32>>()
      .add_push_constant::<[f32; 4]>()
      .add_push_constant::<graphics::Color4>()
      .build()
      .expect("Could not create graphics pipeline");

    Renderer { pipeline }
  }

  pub fn render(&mut self, res: &engine::Resources, commands: &mut graphics::Commands) {
    use crate::ecs::Join;

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

    commands.bind_pipeline(&self.pipeline);
    commands.push_constant(0, &projection);

    let layouts = ecs::read_components::<Layout>(res);
    let backgrounds = ecs::read_components::<Background>(res);

    for (layout, background) in (&layouts, &backgrounds).join() {
      if background.color.a <= 0.0 {
        continue;
      }

      commands.push_constant(1, &[layout.x, layout.y, layout.width, layout.height]);

      commands.push_constant(2, &background.color);

      commands.draw(0..4);
    }
  }
}
