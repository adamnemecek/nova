pub use nova::graphics::pipeline::*;

use super::shader::{self, Shader};
use super::{Color, RenderPass, Vertex};
use nova::math::algebra::Matrix4;
use std::sync::Arc;

pub fn create_default(pass: &Arc<RenderPass>) -> Arc<Pipeline> {
  let device = pass.device();

  // Create a descriptor set layout with just one texture (image and sampler)
  // binding.
  let descriptor_set_layout = DescriptorSetLayout::new().texture().build(device);

  Pipeline::new()
    .render_pass(&pass)
    .vertex_buffer::<Vertex>()
    .push_constant::<Color>()
    .push_constant::<Matrix4<f32>>()
    .descriptor_set_layout(&descriptor_set_layout)
    .vertex_shader(Shader::from_glsl(
      device,
      shader::Kind::Vertex,
      include_str!("shaders/default.vert"),
    ))
    .fragment_shader(Shader::from_glsl(
      device,
      shader::Kind::Fragment,
      include_str!("shaders/default.frag"),
    ))
    .build(device)
}
