use super::backend;
use super::prelude::*;
use super::{Device, RenderPass, ShaderPair};
use std::sync::Arc;

pub struct Pipeline {
  raw: Option<backend::GraphicsPipeline>,
  layout: Option<backend::PipelineLayout>,
  _shaders: ShaderPair,
  device: Arc<Device>,
}

impl Pipeline {
  pub fn new(render_pass: &RenderPass, shaders: ShaderPair) -> Self {
    let device = render_pass.device().clone();

    let vert_entry = backend::ShaderEntryPoint {
      entry: "main",
      module: shaders.vertex.raw(),
      specialization: Default::default(),
    };

    let frag_entry = backend::ShaderEntryPoint {
      entry: "main",
      module: shaders.fragment.raw(),
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
      main_pass: render_pass.raw(),
    };

    let layout = device.raw.create_pipeline_layout(&[], &[]);

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

    let pipeline = device
      .raw
      .create_graphics_pipeline(&pipeline_desc, None)
      .expect("could not create graphics pipeline");

    Pipeline {
      device,
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
    let device = &self.device.raw;

    if let Some(layout) = self.layout.take() {
      device.destroy_pipeline_layout(layout);
    }

    if let Some(pipeline) = self.raw.take() {
      device.destroy_graphics_pipeline(pipeline);
    }
  }
}
