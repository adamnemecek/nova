// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Color, Screen};
use nova_graphics::images::ImageSlice;
use nova_core::math::Rect;
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

  pub fn paint(&mut self, rect: &Rect<f32>, color: Color, image_slice: Option<&ImageSlice>) {
    self
      .render
      .push_constant(&self.pipeline, super::PUSH_CONST_RECT, rect);

    self
      .render
      .push_constant(&self.pipeline, super::PUSH_CONST_TINT, &color);

    match image_slice {
      Some(slice) => {
        let texture_id = self.render.textures_mut().cache_image(slice.image_id);

        self
          .render
          .bind_texture(&self.pipeline, super::DESCRIPTOR_TEXTURE, texture_id);

        self
          .render
          .push_constant(&self.pipeline, super::PUSH_CONST_TEXTURE_RECT, &slice.rect);
      }

      None => {
        self.render.bind_texture(
          &self.pipeline,
          super::DESCRIPTOR_TEXTURE,
          self.render.textures().solid_id(),
        );

        self.render.push_constant(
          &self.pipeline,
          super::PUSH_CONST_TEXTURE_RECT,
          &Rect::unit(),
        );
      }
    };

    self.render.draw(0..4);
  }
}
