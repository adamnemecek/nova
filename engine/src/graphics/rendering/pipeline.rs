use super::backend;
use super::prelude::*;
use super::{Device, RenderPass, Shader, ShaderKind, VertexData};
use std::ops::Range;
use std::sync::Arc;

pub struct Pipeline {
  push_constants: Vec<(gfx_hal::pso::ShaderStageFlags, Range<u32>)>,
  raw: Option<backend::GraphicsPipeline>,
  layout: Option<backend::PipelineLayout>,
  _shaders: PipelineShaderSet,
  device: Arc<Device>,
}

impl Pipeline {
  pub fn new(builder: PipelineBuilder) -> Arc<Self> {
    let render_pass = builder.render_pass.expect("render_pass is required");
    let device = render_pass.device().clone();

    let subpass = gfx_hal::pass::Subpass {
      index: 0,
      main_pass: render_pass.raw(),
    };

    let layout = device
      .raw
      .create_pipeline_layout(&[], &builder.push_constants);

    let mut pipeline_desc = gfx_hal::pso::GraphicsPipelineDesc::new(
      builder.shaders.as_raw(),
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

    pipeline_desc
      .vertex_buffers
      .extend(builder.vertex_buffers.into_iter());

    pipeline_desc
      .attributes
      .extend(builder.vertex_attributes.into_iter());

    let pipeline = device
      .raw
      .create_graphics_pipeline(&pipeline_desc, None)
      .expect("could not create graphics pipeline");

    Arc::new(Pipeline {
      device,
      push_constants: builder.push_constants,
      _shaders: builder.shaders,
      layout: Some(layout),
      raw: Some(pipeline),
    })
  }

  pub fn layout(&self) -> &backend::PipelineLayout {
    self.layout.as_ref().expect("pipeline layout was destroyed")
  }

  pub fn push_constant(&self, index: usize) -> (gfx_hal::pso::ShaderStageFlags, Range<u32>) {
    self.push_constants[index].clone()
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

#[derive(Default)]
pub struct PipelineShaderSet {
  vertex: Option<Shader>,
  fragment: Option<Shader>,
}

impl PipelineShaderSet {
  pub fn load_defaults(device: &Arc<Device>) -> Self {
    PipelineShaderSet {
      vertex: Some(Shader::from_glsl(
        device,
        ShaderKind::Vertex,
        include_str!("shaders/default.vert"),
      )),
      fragment: Some(Shader::from_glsl(
        device,
        ShaderKind::Fragment,
        include_str!("shaders/default.frag"),
      )),
    }
  }

  fn as_raw<'a>(&'a self) -> backend::ShaderSet<'a> {
    fn entry_point(shader: &Option<Shader>) -> Option<backend::ShaderEntryPoint> {
      shader.as_ref().map(|shader| backend::ShaderEntryPoint {
        entry: "main",
        module: shader.raw(),
        specialization: Default::default(),
      })
    };

    gfx_hal::pso::GraphicsShaderSet {
      vertex: entry_point(&self.vertex).expect("vertex shader is required"),
      fragment: entry_point(&self.fragment),
      hull: None,
      domain: None,
      geometry: None,
    }
  }
}

#[derive(Default)]
pub struct PipelineBuilder {
  render_pass: Option<Arc<RenderPass>>,
  shaders: PipelineShaderSet,
  vertex_buffers: Vec<gfx_hal::pso::VertexBufferDesc>,
  vertex_attributes: Vec<gfx_hal::pso::AttributeDesc>,
  push_constants: Vec<(gfx_hal::pso::ShaderStageFlags, Range<u32>)>,
}

impl PipelineBuilder {
  pub fn render_pass(mut self, pass: &Arc<RenderPass>) -> Self {
    self.render_pass = Some(pass.clone());
    self
  }

  pub fn shaders(mut self, shaders: PipelineShaderSet) -> Self {
    self.shaders = shaders;
    self
  }

  pub fn vertex_buffer<T: VertexData>(mut self) -> Self {
    let binding = self.vertex_buffers.len() as u32;

    self.vertex_buffers.push(gfx_hal::pso::VertexBufferDesc {
      binding,
      stride: T::stride(),
      rate: 0,
    });

    let mut offset = 0;

    self
      .vertex_attributes
      .extend(T::attributes().iter().enumerate().map(|(i, attr)| {
        let desc = gfx_hal::pso::AttributeDesc {
          location: i as u32,
          binding,
          element: gfx_hal::pso::Element {
            format: attr.into(),
            offset,
          },
        };

        offset += attr.size();

        desc
      }));

    self
  }

  pub fn push_constant<T>(mut self) -> Self {
    let size = std::mem::size_of::<T>();

    assert!(
      size % 4 == 0,
      "Push constants must be a multiple of 4 bytes in size."
    );

    let start = self.push_constants.last().map(|r| r.1.end).unwrap_or(0);
    let end = start + size as u32 / 4;

    assert!(
      end <= 32,
      "Push constants should not exceed 128 bytes total."
    );

    self
      .push_constants
      .push((gfx_hal::pso::ShaderStageFlags::VERTEX, start..end));

    self
  }

  pub fn build(self) -> Arc<Pipeline> {
    Pipeline::new(self)
  }
}
