// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::text::cache::GlyphCache;
use crate::text::fonts::FontId;
use crate::text::position::PositionedGlyph;
use crate::Color;
use nova_core::math::Rect;
use nova_graphics::images::ImageSlice;
use nova_renderer::{Pipeline, Render, TextureId};

pub struct Canvas<'a, 'b> {
  pub(crate) render: &'a mut Render<'b>,
  pub(crate) pipeline: &'a Pipeline,
}

impl<'a, 'b> Canvas<'a, 'b> {
  pub fn draw_texture(
    &mut self,
    rect: Rect<f32>,
    color: Color,
    texture_id: TextureId,
    texture_rect: Rect<f32>,
  ) {
    self
      .render
      .push_constant(&self.pipeline, super::PUSH_CONST_RECT, &rect);

    self
      .render
      .push_constant(&self.pipeline, super::PUSH_CONST_TINT, &color);

    self
      .render
      .bind_texture(&self.pipeline, super::DESCRIPTOR_TEXTURE, texture_id);

    self.render.push_constant(
      &self.pipeline,
      super::PUSH_CONST_TEXTURE_RECT,
      &texture_rect,
    );

    self.render.draw(0..4);
  }

  pub fn draw_image(&mut self, rect: Rect<f32>, color: Color, image_slice: Option<&ImageSlice>) {
    match image_slice {
      Some(slice) => {
        let texture_id = self.render.textures_mut().cache_image(slice.image_id);

        self.draw_texture(rect, color, texture_id, slice.rect);
      }

      None => {
        self.draw_texture(rect, color, TextureId::SOLID, Rect::unit());
      }
    }
  }

  pub fn draw_cached_glyphs(
    &mut self,
    cache: &mut GlyphCache,
    texture_id: TextureId,
    glyphs: &[(FontId, PositionedGlyph)],
  ) {
    for (font_id, glyph) in glyphs {
      let (tex_coords, coords) = match cache.rect_for(font_id.0, glyph) {
        Ok(Some(x)) => x,
        _ => continue,
      };

      self.draw_texture(
        Rect {
          x1: coords.min.x as f32,
          y1: coords.min.y as f32,
          x2: coords.max.x as f32,
          y2: coords.max.y as f32,
        },
        Color::WHITE,
        texture_id,
        Rect {
          x1: tex_coords.min.x,
          y1: tex_coords.min.y,
          x2: tex_coords.max.x,
          y2: tex_coords.max.y,
        },
      );

      self.render.draw(0..4);
    }
  }
}
