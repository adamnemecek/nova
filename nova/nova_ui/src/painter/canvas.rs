// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Color, Screen};
use nova_graphics as graphics;
use nova_math::Rect;
use nova_renderer as renderer;

pub struct Canvas<'a, 'b> {
  render: &'a mut renderer::Render<'b>,
  pipeline: &'a renderer::Pipeline,
}

impl<'a, 'b> Canvas<'a, 'b> {
  pub(crate) fn new(
    screen: &Screen,
    render: &'a mut renderer::Render<'b>,
    pipeline: &'a renderer::Pipeline,
  ) -> Self {
    render.bind_pipeline(pipeline);
    render.push_constant(pipeline, super::PUSH_CONST_TRANSFORM, screen.projection());

    Self { render, pipeline }
  }

  pub fn paint(&mut self, rect: &Rect<f32>, color: Color, texture: Option<&graphics::ImageSlice>) {
    self
      .render
      .push_constant(&self.pipeline, super::PUSH_CONST_RECT, rect);

    self
      .render
      .push_constant(&self.pipeline, super::PUSH_CONST_TINT, &color);

    self.render.bind_texture_or_default(
      &self.pipeline,
      super::DESCRIPTOR_TEXTURE,
      texture.map(|t| t.image()),
    );

    self.render.push_constant(
      &self.pipeline,
      super::PUSH_CONST_TEXTURE_RECT,
      texture.map(|t| t.rect()).unwrap_or(&Rect::unit()),
    );

    self.render.draw(0..4);
  }
}
