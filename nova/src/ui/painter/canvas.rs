// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Color, Screen};
use crate::graphics;
use crate::math::Rect;
use crate::renderer::{self, Render};

pub struct Canvas<'a, 'b> {
  render: &'a mut Render<'b>,
  default_texture: &'a graphics::ImageSlice,
  pipeline: &'a renderer::Pipeline,
}

impl<'a, 'b> Canvas<'a, 'b> {
  pub(super) fn new(
    screen: &Screen,
    render: &'a mut Render<'b>,
    pipeline: &'a renderer::Pipeline,
    default_texture: &'a graphics::ImageSlice,
  ) -> Self {
    render.bind_pipeline(pipeline);
    render.push_constant(pipeline, super::PUSH_CONST_TRANSFORM, screen.projection());

    Self {
      render,
      default_texture,
      pipeline,
    }
  }

  pub fn paint(
    &mut self,
    rect: &Rect<f32>,
    color: Color,
    texture: Option<&graphics::ImageSlice>,
    texture_id_cache: &mut Option<renderer::TextureId>,
  ) {
    let texture = texture.unwrap_or(self.default_texture);

    self
      .render
      .push_constant(&self.pipeline, super::PUSH_CONST_RECT, rect);

    self
      .render
      .push_constant(&self.pipeline, super::PUSH_CONST_TINT, &color);

    self.render.bind_cached_image(
      &self.pipeline,
      super::DESCRIPTOR_TEXTURE,
      texture.image(),
      texture_id_cache,
    );

    self.render.push_constant(
      &self.pipeline,
      super::PUSH_CONST_TEXTURE_RECT,
      texture.rect(),
    );

    self.render.draw(0..4);
  }
}
