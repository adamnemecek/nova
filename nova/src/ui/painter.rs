// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::layout::ScreenRect;
use super::{Color, Screen, Style};
use crate::ecs;
use crate::el::hierarchy::Hierarchy;
use crate::engine::{self, Engine};
use crate::graphics::{Image, ImageSlice};
use crate::math::{Matrix4, Rect};
use crate::renderer::{self, Render, Renderer};

pub struct Painter {
  pipeline: renderer::Pipeline,
  default_image: ImageSlice,
}

impl Painter {
  pub fn new(engine: &mut Engine, renderer: &Renderer) -> Self {
    ecs::register::<StyleCache>(engine.resources_mut());

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
      .add_descriptor_layout(renderer.texture_descriptor_layout().clone())
      .add_push_constant::<Matrix4<f32>>()
      .add_push_constant::<Rect<f32>>()
      .add_push_constant::<Rect<f32>>()
      .add_push_constant::<Color>()
      .build(renderer.device(), renderer.render_pass())
      .expect("Could not create graphics pipeline");

    let default_image = Image::from_bytes(include_bytes!("./painter/1x1.png"))
      .unwrap()
      .into();

    Painter {
      pipeline,
      default_image,
    }
  }

  pub fn draw(&mut self, render: &mut Render, res: &engine::Resources) {
    let screen = res.fetch::<Screen>();

    render.bind_pipeline(&self.pipeline);
    render.push_constant(&self.pipeline, 0, screen.projection());

    let hierarchy = res.fetch::<Hierarchy>();
    let rects = ecs::read_components::<ScreenRect>(res);
    let styles = ecs::read_components::<Style>(res);
    let mut style_caches = ecs::write_components::<StyleCache>(res);

    for entity in hierarchy.sorted() {
      let (rect, style) = match (rects.get(entity), styles.get(entity)) {
        (Some(rect), Some(style)) if style.bg_color.a > 0.0 => (rect, style),
        _ => continue,
      };

      let style_cache = style_caches
        .entry(entity)
        .unwrap()
        .or_insert_with(StyleCache::default);

      let bg_image = style.bg_image.as_ref().unwrap_or(&self.default_image);

      render.bind_image_cached(
        &self.pipeline,
        0,
        bg_image.image(),
        &mut style_cache.bg_texture,
      );

      render.push_constant(&self.pipeline, 1, rect);
      render.push_constant(&self.pipeline, 2, bg_image.rect());
      render.push_constant(&self.pipeline, 3, &style.bg_color);

      render.draw(0..4);
    }
  }

  pub fn destroy(self, device: &renderer::Device) {
    self.pipeline.destroy(device);
  }
}

#[derive(Debug, Default)]
struct StyleCache {
  bg_texture: Option<renderer::TextureId>,
}

impl ecs::Component for StyleCache {
  type Storage = ecs::BTreeStorage<Self>;
}
