use super::backend;
use super::{Backend, RenderPass, Shader};
use gfx_hal::Device;
use std::sync::Arc;

pub struct Pipeline {
  render_pass: Arc<RenderPass>,
  _shaders: PipelineShaders,
  pipeline: Option<backend::GraphicsPipeline>,
  layout: Option<backend::PipelineLayout>,
  log: bflog::Logger,
}

pub struct PipelineShaders {
  pub vertex: Shader,
  pub fragment: Shader,
}

impl Pipeline {
  pub fn new(render_pass: &Arc<RenderPass>, shaders: PipelineShaders) -> Self {
    let context = render_pass.context();
    let mut log = context.log().with_src("graphics::Pipeline");

    let layout = context.device().create_pipeline_layout(&[], &[]);

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
      main_pass: render_pass.pass(),
    };

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

    let pipeline = context
      .device()
      .create_graphics_pipeline(&pipeline_desc, None)
      .expect("could not create graphics pipeline");

    log.trace("Created.");

    Pipeline {
      render_pass: render_pass.clone(),
      _shaders: shaders,
      pipeline: Some(pipeline),
      layout: Some(layout),
      log,
    }
  }
}

impl Drop for Pipeline {
  fn drop(&mut self) {
    let device = self.render_pass.context().device();

    if let Some(pipeline) = self.pipeline.take() {
      device.destroy_graphics_pipeline(pipeline);
    }

    if let Some(layout) = self.layout.take() {
      device.destroy_pipeline_layout(layout);
    }

    self.log.trace("Destroyed.");
  }
}
