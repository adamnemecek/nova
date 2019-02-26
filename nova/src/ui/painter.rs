// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::layout::ScreenRect;
use super::{Color, Style, StyleCache};
use crate::ecs;
use crate::el::hierarchy::Hierarchy;
use crate::engine;
use crate::graphics::Image;
use crate::math::Matrix4;
use crate::renderer::{self, Render, Renderer};
use crate::window::Window;

pub struct Painter {
  pipeline: renderer::Pipeline,
  default_image: Image,
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
      .add_descriptor_layout(renderer.texture_descriptor_layout().clone())
      .add_push_constant::<Matrix4<f32>>()
      .add_push_constant::<[f32; 4]>()
      .add_push_constant::<Color>()
      .build(renderer.device(), renderer.render_pass())
      .expect("Could not create graphics pipeline");

    let default_image = Image::from_bytes(include_bytes!("1x1.png")).unwrap();

    Painter {
      pipeline,
      default_image,
    }
  }

  pub fn draw(&mut self, render: &mut Render, res: &engine::Resources) {
    // Scale the entire UI based on the size of the window.
    let size = res.fetch::<Window>().size();

    let scale = if size.height > size.width {
      (size.width / 1280).max(1) as f32
    } else {
      (size.height / 720).max(1) as f32
    };

    // Create a projection matrix that converts UI units to screen space.
    let projection =
      Matrix4::new_orthographic(0.0, size.width as f32, 0.0, size.height as f32, -1.0, 1.0)
        .prepend_scaling(scale);

    render.bind_pipeline(&self.pipeline);
    render.push_constant(&self.pipeline, 0, &projection);

    let hierarchy = res.fetch::<Hierarchy>();
    let rects = ecs::read_components::<ScreenRect>(res);
    let styles = ecs::read_components::<Style>(res);
    let mut style_caches = ecs::write_components::<StyleCache>(res);

    for entity in hierarchy.sorted() {
      let (rect, style) = match (rects.get(entity), styles.get(entity)) {
        (Some(rect), Some(style)) if style.bg_color.a > 0.0 => (rect, style),
        x => {
          dbg!(x);
          continue;
        }
      };

      let style_cache = style_caches
        .entry(entity)
        .unwrap()
        .or_insert_with(StyleCache::default);

      let bg_image = style.bg_image.as_ref().unwrap_or(&self.default_image);

      render.bind_image_cached(&self.pipeline, 0, bg_image, &mut style_cache.bg_texture);

      render.push_constant(
        &self.pipeline,
        1,
        &[rect.left, rect.top, rect.size.width, rect.size.height],
      );

      render.push_constant(&self.pipeline, 2, &style.bg_color);

      render.draw(0..4);
    }
  }

  pub fn destroy(self, device: &renderer::Device) {
    self.pipeline.destroy(device);
  }
}
