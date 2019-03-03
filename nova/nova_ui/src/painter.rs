// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod canvas;

pub use self::canvas::Canvas;

use super::layout::ScreenRect;
use super::{Color, Screen, Style};
use nova_core::ecs;
use nova_core::el::hierarchy::Hierarchy;
use nova_core::engine;
use nova_math::{Matrix4, Rect};
use nova_renderer::{self as renderer, Render, Renderer};

const DESCRIPTOR_TEXTURE: usize = 0;

const PUSH_CONST_TRANSFORM: usize = 0;
const PUSH_CONST_RECT: usize = 1;
const PUSH_CONST_TEXTURE_RECT: usize = 2;
const PUSH_CONST_TINT: usize = 3;

pub struct Painter {
  pipeline: renderer::Pipeline,
}

impl Painter {
  pub fn new(renderer: &Renderer) -> Self {
    let vertex_shader = renderer::Shader::new(
      renderer.device(),
      &renderer::shader::Spirv::from_glsl(
        renderer::ShaderKind::Vertex,
        include_str!("./painter/shaders/panels.vert"),
      ),
    );

    let fragment_shader = renderer::Shader::new(
      renderer.device(),
      &renderer::shader::Spirv::from_glsl(
        renderer::ShaderKind::Fragment,
        include_str!("./painter/shaders/panels.frag"),
      ),
    );

    let pipeline = renderer::PipelineBuilder::new()
      .set_vertex_shader(&vertex_shader)
      .set_fragment_shader(&fragment_shader)
      .add_descriptor_layout(renderer.textures().descriptor_layout().clone())
      .add_push_constant::<Matrix4<f32>>()
      .add_push_constant::<Rect<f32>>()
      .add_push_constant::<Rect<f32>>()
      .add_push_constant::<Color>()
      .build(renderer.device(), renderer.render_pass())
      .expect("Could not create graphics pipeline");

    Painter { pipeline }
  }

  pub fn draw(&mut self, render: &mut Render, res: &engine::Resources) {
    let screen = res.fetch::<Screen>();
    let mut canvas = Canvas::new(&screen, render, &self.pipeline);

    let hierarchy = res.fetch::<Hierarchy>();
    let rects = ecs::read_components::<ScreenRect>(res);
    let styles = ecs::read_components::<Style>(res);

    for entity in hierarchy.sorted() {
      let (rect, style) = match (rects.get(entity), styles.get(entity)) {
        (Some(rect), Some(style)) if style.bg_color.a > 0.0 => (rect, style),
        _ => continue,
      };

      canvas.paint(rect, style.bg_color, style.bg_image.as_ref());
    }
  }

  pub fn destroy(self, device: &renderer::Device) {
    self.pipeline.destroy(device);
  }
}
