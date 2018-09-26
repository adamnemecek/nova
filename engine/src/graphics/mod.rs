// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `graphics` module handles basic drawing and image data.
//!
//! This module requires an engine context with an `engine::Window`, which it
//! uses to create a `Canvas` for drawing to the window. The canvas can be used
//! to draw graphics primitives such as an `Image`, which can be loaded from an
//! image asset file.
//!
//! This module also creates a `DrawLayers` resource which stores `DrawLayer`
//! implementations and a `LayerDrawer` engine process which draws the layers
//! in that resource. Other modules can add draw layers to receive access to the
//! `Canvas` once a frame to draw.
//!
//! The `Atlas` struct provides a wrapper around an `Image` asset that slices
//! it into cells, for use with tile sets or sprite sheets.

use crate::prelude::*;
use crate::window::Window;

pub use ggez::graphics::{Color, DrawParam as DrawParams};

pub mod panels;

mod atlas;
mod image;
mod rendering;

pub use self::atlas::*;
pub use self::image::*;
pub use self::rendering::Canvas;

use self::rendering::{RenderTarget, Renderer};

pub struct Extension {
  renderer: Option<Box<Renderer>>,
  render_target: Option<Box<RenderTarget>>,
}

impl engine::Extension for Extension {
  fn after_tick(&mut self, ctx: &mut engine::Context) {
    let mut renderer = self.renderer.take().unwrap();
    let mut render_target = self.render_target.take().unwrap();

    // Resize render target to match window size.
    {
      let window = engine::fetch_resource::<Window>(ctx);

      let size = window.size();
      let size = Vector2::new(size.x.round() as u32, size.y.round() as u32);

      if window.was_resized() {
        render_target.destroy(&renderer);
        render_target = Box::new(RenderTarget::new(&mut renderer, size));
      }
    }

    rendering::render(&mut renderer, &mut render_target, |canvas| {
      println!("gottem");

      let root = panels::get_root(ctx);

      if let Some(root) = root {
        panels::draw(ctx, canvas, root);
      }
    });

    // Clear canvas to eigengrau.
    // self.canvas.clear(Color::new(0.086, 0.086, 0.114, 1.0));

    // Draw root panel and its children.
    /*

    self.canvas.present();
    */

    self.renderer = Some(renderer);
    self.render_target = Some(render_target);
  }

  fn on_exit(&mut self, _ctx: &mut engine::Context) {
    let renderer = self.renderer.take().unwrap();
    let render_target = self.render_target.take().unwrap();

    render_target.destroy(&renderer);
    renderer.destroy();
  }
}

/// Initialize graphics for the given engine context. Requires a window.
pub fn init(ctx: &mut engine::Context) {
  let extension = {
    let window = engine::fetch_resource::<Window>(ctx);

    let size = window.size();
    let size = Vector2::new(size.x.round() as u32, size.y.round() as u32);

    let mut renderer = Renderer::new(window.as_winit());
    let render_target = RenderTarget::new(&mut renderer, size);

    Extension {
      renderer: Some(Box::new(renderer)),
      render_target: Some(Box::new(render_target)),
    }
  };

  engine::add_extension(ctx, extension);

  panels::init(ctx);
}
