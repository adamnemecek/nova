// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Struct for drawing to a `Window`.
pub struct Canvas {
  ctx: ggez::Context,
  size: Vector2<f32>,
}

impl Canvas {
  /// Creates a new canvas from the given window.
  pub fn new(window: &engine::Window) -> Canvas {
    let mut ctx = window.ctx.take().expect("window already has a canvas");
    let rect = ggez::graphics::screen_coordinates(&mut ctx);

    Canvas {
      ctx,
      size: Vector2::new(rect.w, rect.h),
    }
  }

  /// Gets the size of the canvas in pixels.
  pub fn size(&self) -> Vector2<f32> {
    self.size
  }

  /// Resizes the canvas to the given size in pixels.
  pub fn resize(&mut self, size: Vector2<f32>) {
    ggez::graphics::set_screen_coordinates(&mut self.ctx, Rect::new(0.0, 0.0, size.x, size.y))
      .expect("could not set screen coordinates");

    self.size = size;
  }

  /// Clear the canvas with the given color.
  pub fn clear(&mut self, color: Color) {
    ggez::graphics::clear(&mut self.ctx, color);
  }

  /// Push a transform so that all subsequent draw calls have it applied.
  pub fn push_transform(&mut self, transform: Matrix4<f32>) {
    ggez::graphics::push_transform(&mut self.ctx, Some(transform));
    ggez::graphics::apply_transformations(&mut self.ctx).expect("could not push transform");
  }

  /// Draws an image on the canvas.
  pub fn draw(&mut self, image: &Image, params: DrawParams) -> ggez::GameResult<()> {
    let mut inner = image.ggez_image.lock().expect("could not lock Image::ggez");

    // Create the ggez image from the loaded image data if it has not yet been
    // created.
    if !inner.is_some() {
      let size = image.size();

      let mut ggez_image = ggez::graphics::Image::from_rgba8(
        &mut self.ctx,
        size.x as u16,
        size.y as u16,
        &image.rgba_image,
      )?;

      ggez_image.set_filter(ggez::graphics::FilterMode::Nearest);

      *inner = Some(ggez_image);
    }

    ggez::graphics::draw(&mut self.ctx, inner.as_ref().unwrap(), params)
  }

  /// Pops the most recent transform applied with `push_transform`.
  pub fn pop_transform(&mut self) {
    ggez::graphics::pop_transform(&mut self.ctx);
    ggez::graphics::apply_transformations(&mut self.ctx).expect("could not pop transform");
  }

  /// Presents the canvas, showing what has been drawn since the last `clear`.
  pub fn present(&mut self) {
    ggez::graphics::present(&mut self.ctx).expect("could not present");
  }
}
