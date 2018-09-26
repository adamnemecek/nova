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

pub mod panels;

mod atlas;
mod backend;
mod canvas;
mod device;
mod image;

pub use self::atlas::*;
pub use self::canvas::*;
pub use self::image::*;
pub use ggez::graphics::{Color, DrawParam as DrawParams};

pub struct Extension {
  device: Option<device::Context>,
}

impl engine::Extension for Extension {
  fn after_tick(&mut self, _ctx: &mut engine::Context) {
    // Resize canvas to match window size.
    /*
    {
      let window = engine::fetch_resource::<engine::Window>(ctx);

      if window.was_resized() {
        self.canvas.resize(window.size());
      }
    }
    */

    // Clear canvas to eigengrau.
    // self.canvas.clear(Color::new(0.086, 0.086, 0.114, 1.0));

    // Draw root panel and its children.
    /*
    let root = panels::get_root(ctx);

    if let Some(root) = root {
      panels::draw(ctx, &mut self.canvas, root);
    }

    self.canvas.present();
    */
  }

  fn on_exit(&mut self, _ctx: &mut engine::Context) {
    self.device.take().expect("no device to destroy").destroy();
  }
}

/// Initialize graphics for the given engine context. Requires a window.
pub fn init(ctx: &mut engine::Context) {
  let device = {
    let window = engine::fetch_resource::<Window>(ctx);

    device::Context::new(window.as_winit())
  };

  engine::add_extension(
    ctx,
    Extension {
      device: Some(device),
    },
  );

  panels::init(ctx);
}
