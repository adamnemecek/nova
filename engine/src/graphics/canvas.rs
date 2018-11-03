use super::rendering;
use crate::prelude::*;
use std::sync::Arc;

pub struct Canvas {
  size: Vector2<f32>,
  renderer: rendering::Renderer,
  swapchain: rendering::Swapchain,
  log: bflog::Logger,
}

impl Canvas {
  pub fn new(
    device: &Arc<rendering::Device>,
    window: &winit::Window,
    log: &bflog::Logger,
  ) -> Canvas {
    let mut log = log.with_src("nova::graphics::Canvas");

    let render_pass = rendering::RenderPass::new(&device);
    let renderer = rendering::Renderer::new(&device, &render_pass);

    log.trace("Created renderer.");

    let mut canvas = Canvas {
      size: Vector2::zeros(),
      renderer,
      swapchain: rendering::Swapchain::new(&device),
      log,
    };

    canvas.resize_to_fit(window);

    canvas
  }

  pub fn resize_to_fit(&mut self, window: &winit::Window) {
    let size = window
      .get_inner_size()
      .expect("window is destroyed")
      .to_physical(window.get_hidpi_factor());

    let size = Vector2::new(size.width as f32, size.width as f32);

    if size == self.size {
      return;
    }

    self.size = size;

    self.destroy_swapchain("resize_to_fit", "window resized");
  }

  pub fn begin(&mut self) {
    self.ensure_swapchain();

    match self.renderer.begin(&mut self.swapchain) {
      Err(rendering::BeginRenderError::SwapchainOutOfDate) => {
        self.destroy_swapchain("begin", "out of date");
        self.begin();

        return;
      }

      Err(rendering::BeginRenderError::SurfaceLost) => {
        panic!("surface lost");
      }

      Ok(_) => {}
    };
  }

  pub fn present(&mut self) {
    match self.renderer.present(&mut self.swapchain) {
      Err(rendering::PresentError::SwapchainOutOfDate) => {
        self.destroy_swapchain("present", "out of date");
        return;
      }

      Ok(_) => {}
    }
  }

  fn ensure_swapchain(&mut self) {
    if !self.swapchain.is_destroyed() {
      return;
    }

    self.swapchain.create(
      self.renderer.pass(),
      self.size.x.round() as u32,
      self.size.y.round() as u32,
    );

    self
      .log
      .trace("Created swapchain.")
      .with("width", &self.swapchain.width())
      .with("height", &self.swapchain.height());
  }

  fn destroy_swapchain(&mut self, when: &'static str, reason: &'static str) {
    if self.swapchain.is_destroyed() {
      return;
    }

    self.swapchain.destroy();

    self
      .log
      .trace("Destroyed swapchain.")
      .with("when", &when)
      .with("reason", &reason);
  }
}
