// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod canvas;

pub use self::canvas::Canvas;

use crate::image::Image;
use crate::nodes;
use crate::screen::ScreenRect;
use crate::text::cache::GlyphCache;
use crate::text::position::PositionedText;
use crate::{Color, Screen};
use nova_core::components;
use nova_core::math::{Matrix4, Rect, Size};
use nova_core::resources::Resources;
use nova_renderer::images::DeviceImageFormat;
use nova_renderer::{self as renderer, Render, Renderer, TextureId};

const DESCRIPTOR_TEXTURE: usize = 0;

pub struct Painter {
  image_pipeline: renderer::Pipeline,
  text_pipeline: renderer::Pipeline,
  glyph_cache_texture: TextureId,
}

impl Painter {
  pub fn new(renderer: &mut Renderer) -> Self {
    let vertex_shader = renderer::Shader::new(
      renderer.device(),
      &renderer::shader::Spirv::from_glsl(
        renderer::ShaderKind::Vertex,
        include_str!("./painter/shaders/quad.vert"),
      ),
    );

    let image_shader = renderer::Shader::new(
      renderer.device(),
      &renderer::shader::Spirv::from_glsl(
        renderer::ShaderKind::Fragment,
        include_str!("./painter/shaders/image.frag"),
      ),
    );

    let text_shader = renderer::Shader::new(
      renderer.device(),
      &renderer::shader::Spirv::from_glsl(
        renderer::ShaderKind::Fragment,
        include_str!("./painter/shaders/text.frag"),
      ),
    );

    let image_pipeline = renderer::PipelineBuilder::new()
      .set_vertex_shader(&vertex_shader)
      .set_fragment_shader(&image_shader)
      .add_descriptor_layout(renderer.textures().descriptor_layout().clone())
      .set_push_constants::<(Matrix4<f32>, Rect<f32>, Rect<f32>, Color)>()
      .build(renderer.device(), renderer.render_pass())
      .expect("Could not create image pipeline");

    let text_pipeline = renderer::PipelineBuilder::new()
      .set_vertex_shader(&vertex_shader)
      .set_fragment_shader(&text_shader)
      .add_descriptor_layout(renderer.textures().descriptor_layout().clone())
      .set_push_constants::<(Matrix4<f32>, Rect<f32>, Rect<f32>, Color)>()
      .build(renderer.device(), renderer.render_pass())
      .expect("Could not create text pipeline");

    let glyph_cache_texture =
      renderer.create_texture(Size::new(1024, 1024), DeviceImageFormat::R8Unorm);

    renderer
      .textures_mut()
      .clear_texture(glyph_cache_texture, Color::TRANSPARENT);

    Painter {
      image_pipeline,
      text_pipeline,
      glyph_cache_texture,
    }
  }

  pub fn draw(&mut self, render: &mut Render, res: &Resources) {
    let screen = res.fetch::<Screen>();
    let nodes = nodes::borrow(res);
    let mut glyph_cache = res.fetch_mut::<GlyphCache>();

    let rects = components::borrow::<ScreenRect>(res);
    let images = components::borrow::<Image>(res);
    let texts = components::borrow::<PositionedText>(res);

    glyph_cache
      .cache_queued(|rect, bytes| {
        render.textures_mut().copy_to_texture(
          self.glyph_cache_texture,
          Rect {
            x1: rect.min.x,
            y1: rect.min.y,
            x2: rect.max.x,
            y2: rect.max.y,
          },
          bytes,
        )
      })
      .expect("Could not update glyph cache texture");

    let mut canvas = Canvas {
      render,
      image_pipeline: &self.image_pipeline,
      text_pipeline: &self.text_pipeline,
      screen: &screen,
    };

    canvas.draw_texture(
      true,
      Rect {
        x1: 0.0,
        y1: 0.0,
        x2: 1024.0,
        y2: 1024.0,
      },
      [0.0, 1.0, 0.0, 0.85].into(),
      self.glyph_cache_texture,
      Rect {
        x1: 0.0,
        y1: 0.0,
        x2: 1.0,
        y2: 1.0,
      },
    );

    for entity in nodes.sorted() {
      if let (Some(rect), Some(image)) = (rects.get(entity), images.get(entity)) {
        canvas.draw_image(rect.0, Color::WHITE, &image.slice);
      }

      if let Some(text) = texts.get(entity) {
        canvas.draw_cached_glyphs(&mut glyph_cache, self.glyph_cache_texture, &text.glyphs);
      }
    }
  }

  pub fn destroy(self, device: &renderer::Device) {
    self.image_pipeline.destroy(device);
    self.text_pipeline.destroy(device);
  }
}
