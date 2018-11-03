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

//pub mod panels;

mod atlas;
mod canvas;
mod image;
mod rendering;

pub use self::atlas::*;
pub use self::canvas::Canvas;
pub use self::image::*;

pub struct Extension {
  canvas: Canvas,
}

impl engine::Extension for Extension {
  fn after_tick(&mut self, ctx: &mut engine::Context) {
    {
      let window = engine::fetch_resource::<Window>(ctx);

      if window.was_resized() {
        self.canvas.resize_to_fit(window.as_winit());
      }

      self.canvas.begin();

      self.canvas.present();
    }
  }
}

/// Initialize graphics for the given engine context. Requires a window.
pub fn init(ctx: &mut engine::Context, log: &bflog::Logger) {
  let mut log = log.with_src("nova::graphics");

  engine::add_extension(ctx, {
    let window = engine::fetch_resource::<Window>(ctx);
    let window = window.as_winit();

    let device = rendering::init(window).expect("could not create device");

    log.trace("Created device");

    let canvas = Canvas::new(&device, window, &log);

    Extension { canvas }
  });

  //panels::init(ctx);
}
