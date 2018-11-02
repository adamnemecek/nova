use super::backend;
use super::mesh::Vertex;
use super::{Backend, RenderPass, RenderTarget, Shader};
use gfx_hal::Device;
use std::sync::Arc;

pub struct Pipeline {
  _shaders: ShaderSet,
  raw: Option<backend::GraphicsPipeline>,
  layout: Option<backend::PipelineLayout>,
  render_pass: Arc<RenderPass>,
}

pub struct ShaderSet {
  pub vertex: Shader,
  pub fragment: Shader,
}

impl Pipeline {
  pub fn new(render_target: &RenderTarget, shaders: ShaderSet) -> Self {
    let device = &render_target.context.device;

    let vert_entry = gfx_hal::pso::EntryPoint::<Backend> {
      entry: "main",
      module: shaders.vertex.module(),
      specialization: Default::default(),
    };

    let frag_entry = gfx_hal::pso::EntryPoint::<Backend> {
      entry: "main",
      module: shaders.fragment.module(),
      specialization: Default::default(),
    };

    let shader_entries = gfx_hal::pso::GraphicsShaderSet {
      vertex: vert_entry,
      hull: None,
      domain: None,
      geometry: None,
      fragment: Some(frag_entry),
    };

    let subpass = gfx_hal::pass::Subpass {
      index: 0,
      main_pass: render_target.render_pass.raw(),
    };

    let layout = device.create_pipeline_layout(&[], &[]);

    let mut pipeline_desc = gfx_hal::pso::GraphicsPipelineDesc::new(
      shader_entries,
      gfx_hal::Primitive::TriangleList,
      gfx_hal::pso::Rasterizer::FILL,
      &layout,
      subpass,
    );

    pipeline_desc
      .blender
      .targets
      .push(gfx_hal::pso::ColorBlendDesc(
        gfx_hal::pso::ColorMask::ALL,
        gfx_hal::pso::BlendState::ALPHA,
      ));

    Vertex::add_pipeline_binding(&mut pipeline_desc, 0);

    let pipeline = device
      .create_graphics_pipeline(&pipeline_desc, None)
      .expect("could not create graphics pipeline");

    Pipeline {
      render_pass: render_target.render_pass.clone(),
      _shaders: shaders,
      layout: Some(layout),
      raw: Some(pipeline),
    }
  }

  pub fn raw(&self) -> &backend::GraphicsPipeline {
    self.raw.as_ref().expect("pipeline was destroyed")
  }
}

impl Drop for Pipeline {
  fn drop(&mut self) {
    let device = &self.render_pass.context().device;

    if let Some(layout) = self.layout.take() {
      device.destroy_pipeline_layout(layout);
    }

    if let Some(pipeline) = self.raw.take() {
      device.destroy_graphics_pipeline(pipeline);
    }
  }
}
