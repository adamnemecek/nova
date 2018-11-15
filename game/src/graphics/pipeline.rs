pub use nova::graphics::pipeline::*;

use super::{Color4, RenderPass, Vertex};
use nova::math::Matrix4;
use std::sync::Arc;

pub fn create_default(pass: &Arc<RenderPass>) -> Arc<Pipeline> {
  let device = pass.device();

  let descriptor_layout = DescriptorLayout::new(&device, &[descriptor::Binding::Texture]);

  Pipeline::new()
    .render_pass(&pass)
    .vertex_buffer::<Vertex>()
    .push_constant::<Color4>()
    .push_constant::<Matrix4<f32>>()
    .descriptor_layout(&descriptor_layout)
    .shaders(ShaderSet {
      vertex: shader::EntryPoint(
        Shader::new(
          device,
          &shader::Spirv::from_glsl(ShaderKind::Vertex, include_str!("shaders/default.vert")),
        ),
        "main",
      ),
      fragment: Some(shader::EntryPoint(
        Shader::new(
          device,
          &shader::Spirv::from_glsl(ShaderKind::Fragment, include_str!("shaders/default.frag")),
        ),
        "main",
      )),
    })
    .build(device)
}
