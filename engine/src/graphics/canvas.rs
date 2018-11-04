use super::mesh;
use super::rendering;
use super::Mesh;
use crate::math::{Matrix4, Orthographic3, Transform3};
use crate::prelude::*;
use std::sync::Arc;

pub struct Canvas {
  size: Vector2<f32>,
  projection: Matrix4<f32>,
  renderer: rendering::Renderer,
  swapchain: rendering::Swapchain,
  pipeline: Arc<rendering::Pipeline>,
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
    let renderer = rendering::Renderer::new(&render_pass);

    log.trace("Created renderer.");

    let shaders = rendering::PipelineShaderSet::load_defaults(device);

    log.trace("Loaded default pipeline shaders.");

    let pipeline = rendering::PipelineBuilder::default()
      .render_pass(&render_pass)
      .shaders(shaders)
      .vertex_buffer::<mesh::Vertex>()
      .push_constant::<Vector4<f32>>()
      .push_constant::<Transform3<f32>>()
      .build();

    log.trace("Created pipeline.");

    let mut canvas = Canvas {
      size: Vector2::zeros(),
      projection: Matrix4::identity(),
      renderer,
      pipeline,
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

    let half = size / 2.0;

    self.size = size;
    self.projection =
      Orthographic3::new(-half.x, half.x, -half.y, half.y, -1.0, 1.0).to_homogeneous();

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

    self.renderer.bind_pipeline(&self.pipeline);

    self.set_transform(Matrix4::identity());
    self.set_tint([1.0, 1.0, 1.0, 1.0]);
  }

  pub fn set_tint(&mut self, tint: [f32; 4]) {
    self.renderer.push_constant(0, &tint);
  }

  pub fn set_transform(&mut self, transform: Matrix4<f32>) {
    let transform = transform * self.projection;

    self.renderer.push_constant(1, &transform);
  }

  pub fn draw(&mut self, mesh: &Mesh) {
    self.renderer.bind_vertex_buffer(0, mesh.vertex_buffer());
    self.renderer.bind_index_buffer(mesh.index_buffer());
    self.renderer.draw_indexed(mesh.indices());
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
