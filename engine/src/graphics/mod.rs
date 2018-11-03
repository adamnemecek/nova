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
mod image;
mod rendering;

pub use self::atlas::*;
pub use self::image::*;

pub struct Extension {
  swapchain: rendering::Swapchain,
  renderer: rendering::Renderer,
}

impl engine::Extension for Extension {
  fn after_tick(&mut self, ctx: &mut engine::Context) {
    {
      let window = engine::fetch_resource::<Window>(ctx);

      if window.was_resized() {
        self.swapchain.destroy();
      }

      if self.swapchain.is_destroyed() {
        let width = window.size().x.round() as u32;
        let height = window.size().y.round() as u32;

        self.swapchain.create(self.renderer.pass(), width, height);
      }

      match self.renderer.begin(&mut self.swapchain) {
        Err(rendering::BeginRenderError::SwapchainOutOfDate) => {
          self.swapchain.destroy();
          return;
        }

        Err(rendering::BeginRenderError::SurfaceLost) => {
          panic!("surface lost");
        }

        Ok(_) => {}
      }

      match self.renderer.present(&mut self.swapchain) {
        Err(rendering::PresentError::SwapchainOutOfDate) => {
          self.swapchain.destroy();
          return;
        }

        Ok(_) => {}
      }
    }
  }
}

/// Initialize graphics for the given engine context. Requires a window.
pub fn init(ctx: &mut engine::Context) {
  let window = engine::fetch_resource::<Window>(ctx);
  let device = rendering::init(window.as_winit()).unwrap();

  drop(window);

  let swapchain = rendering::Swapchain::new(&device);

  let render_pass = rendering::RenderPass::new(&device);
  let renderer = rendering::Renderer::new(&device, &render_pass);

  engine::add_extension(
    ctx,
    Extension {
      swapchain,
      renderer,
    },
  );

  //panels::init(ctx);
}
