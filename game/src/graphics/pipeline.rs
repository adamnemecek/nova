use super::rendering::{DescriptorSetLayout, Pipeline, RenderPass, Shader, ShaderKind};
use super::{Color, Vertex};
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
      ShaderKind::Vertex,
      include_str!("shaders/default.vert"),
    ))
    .fragment_shader(Shader::from_glsl(
      device,
      ShaderKind::Fragment,
      include_str!("shaders/default.frag"),
    ))
    .build(device)
}
