pub use nova::graphics::pipeline::*;

use super::shader::{self, Shader};
use super::{Color, RenderPass, Vertex};
use nova::math::Matrix4;
use std::sync::Arc;

pub fn create_default(pass: &Arc<RenderPass>) -> Arc<Pipeline> {
  let device = pass.device();

  let descriptor_layout = DescriptorLayout::new(&device, &[descriptor::Binding::Texture]);

  Pipeline::new()
    .render_pass(&pass)
    .vertex_buffer::<Vertex>()
    .push_constant::<Color>()
    .push_constant::<Matrix4<f32>>()
    .descriptor_layout(&descriptor_layout)
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
