use super::Vertex;
use nova::graphics::render::descriptor::{DescriptorBinding, DescriptorLayout};
use nova::graphics::render::pipeline::{self, Pipeline, PipelineBuilder};
use nova::graphics::render::shader::{self, Shader, ShaderKind};
use nova::graphics::render::RenderPass;
use nova::graphics::Color4;
use nova::math::Matrix4;
use std::sync::Arc;

pub fn create_default(pass: &Arc<RenderPass>) -> Result<Arc<Pipeline>, pipeline::BuildError> {
  let device = pass.device();

  let descriptor_layout = DescriptorLayout::new(&device, &[DescriptorBinding::Texture]);

  let vertex_shader = Shader::new(
    device,
    &shader::Spirv::from_glsl(ShaderKind::Vertex, include_str!("shaders/default.vert")),
  );

  let fragment_shader = Shader::new(
    device,
    &shader::Spirv::from_glsl(ShaderKind::Fragment, include_str!("shaders/default.frag")),
  );

  PipelineBuilder::new()
    .set_render_pass(&pass)
    .set_vertex_shader(&Arc::new(vertex_shader), "main")
    .set_fragment_shader(&Arc::new(fragment_shader), "main")
    .add_vertex_buffer::<Vertex>()
    .add_push_constant::<Color4>()
    .add_push_constant::<Matrix4<f32>>()
    .add_descriptor_layout(&Arc::new(descriptor_layout))
    .build()
    .map(Arc::new)
}
