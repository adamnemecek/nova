use super::mesh;
use super::rendering;
use super::{image::Image, Color, Mesh, Vertex};
use crate::math;
use crate::math::Matrix4;
use crate::prelude::*;
use std::sync::Arc;

pub struct Canvas {
  size: Vector2<f32>,
  projection: Matrix4<f32>,
  quad: Mesh,
  renderer: rendering::Renderer,
  swapchain: rendering::Swapchain,
  pipeline: Arc<rendering::Pipeline>,
  descriptor_set: rendering::DescriptorSet,
  _image: Image,
  _sampler: rendering::TextureSampler,
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

    let image = Image::new(device, include_bytes!("do-it.jpg"));
    let sampler = rendering::TextureSampler::new(device);

    let descriptor_set_layout = rendering::DescriptorSetLayout::new()
      .texture()
      .create(&device);

    let descriptor_pool = rendering::DescriptorPool::new(&descriptor_set_layout, 1);

    let descriptor_set = rendering::DescriptorSet::new(
      &descriptor_pool,
      &[rendering::Descriptor::SampledTexture(
        image.texture(),
        &sampler,
      )],
    );

    let pipeline = rendering::Pipeline::new()
      .render_pass(&render_pass)
      .shaders(shaders)
      .vertex_buffer::<mesh::Vertex>()
      .push_constant::<Color>()
      .push_constant::<Matrix4<f32>>()
      .descriptor_set_layout(&descriptor_set_layout)
      .create();

    log.trace("Created pipeline.");

    let quad = Mesh::new(
      device,
      &[
        Vertex::new([-0.5, -0.5], [1.0, 1.0, 1.0, 1.0], [1.0, 0.0]),
        Vertex::new([0.5, -0.5], [1.0, 1.0, 1.0, 1.0], [0.0, 0.0]),
        Vertex::new([0.5, 0.5], [1.0, 1.0, 1.0, 1.0], [0.0, 1.0]),
        Vertex::new([-0.5, 0.5], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0]),
      ],
      &[0, 1, 2, 2, 3, 0],
    );

    log.trace("Created quad mesh.");

    let mut canvas = Canvas {
      size: Vector2::zeros(),
      projection: Matrix4::identity(),
      renderer,
      pipeline,
      quad,
      swapchain: rendering::Swapchain::new(&device),
      descriptor_set,
      _image: image,
      _sampler: sampler,
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

    let size = Vector2::new(size.width as f32, size.height as f32);

    if size == self.size {
      return;
    }

    self.size = size;
    self.projection = math::Matrix4::new_orthographic(0.0, size.x, 0.0, size.y, -1.0, 1.0);

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
    self.renderer.bind_descriptor_set(0, &self.descriptor_set);

    self.set_transform(Matrix4::identity());
    self.set_tint(Color::WHITE);
  }

  pub fn set_tint(&mut self, color: Color) {
    self.renderer.push_constant(0, &color);
  }

  pub fn set_transform(&mut self, transform: Matrix4<f32>) {
    let transform = self.projection * transform;

    self.renderer.push_constant(1, &transform);
  }

  pub fn draw_quad(&mut self) {
    self
      .renderer
      .bind_vertex_buffer(0, self.quad.vertex_buffer());

    self.renderer.bind_index_buffer(self.quad.index_buffer());

    self.renderer.draw_indexed(self.quad.indices());
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
