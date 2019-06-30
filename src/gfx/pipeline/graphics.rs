// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
pub use gfx_hal::pso::PipelineStage;

/// Represents a graphics pipeline object on the device.
pub struct Graphics {
  context: Arc<Context>,
  pipeline: Expect<backend::Pipeline>,
  layout: Expect<backend::PipelineLayout>,
  push_constant_count: usize,
  descriptor_layouts: Vec<DescriptorLayout>,
  _shaders: ShaderSet,
}

impl Graphics {
  /// Creates a new graphics pipeline object from the given [`Builder`].
  pub fn new(context: &Arc<Context>, builder: PipelineBuilder) -> Result<Self, GraphicsError> {
    debug_assert!(
      builder.size_of_push_constants % 4 == 0,
      "size_of_push_constants must be a multiple of 4"
    );

    let push_constant_count = builder.size_of_push_constants / 4;

    let render_pass = builder.render_pass.as_ref().ok_or(GraphicsError::NoRenderPass)?;

    let descriptor_layouts = builder.desriptor_layouts.iter().map(DescriptorLayout::as_backend);

    let push_constant_ranges = if builder.size_of_push_constants > 0 {
      Some((gfx_hal::pso::ShaderStageFlags::ALL, 0..push_constant_count as u32))
    } else {
      None
    };

    let layout = unsafe {
      context
        .device()
        .create_pipeline_layout(descriptor_layouts, push_constant_ranges)
        .map_err(CreationError::from)?
    };

    let mut desc = gfx_hal::pso::GraphicsPipelineDesc::new(
      gfx_hal::pso::GraphicsShaderSet {
        vertex: builder
          .shaders
          .vertex
          .as_ref()
          .ok_or(GraphicsError::NoVertexShader)?
          .backend_entrypoint(),

        fragment: builder.shaders.fragment.as_ref().map(shader::Module::backend_entrypoint),

        geometry: None,
        domain: None,
        hull: None,
      },
      gfx_hal::Primitive::TriangleStrip,
      gfx_hal::pso::Rasterizer::FILL,
      &layout,
      gfx_hal::pass::Subpass { index: 0, main_pass: render_pass.as_backend() },
    );

    desc.blender.targets.push(gfx_hal::pso::ColorBlendDesc(
      gfx_hal::pso::ColorMask::ALL,
      gfx_hal::pso::BlendState::ALPHA,
    ));

    desc.vertex_buffers.extend(builder.vertex_buffers);
    desc.attributes.extend(builder.vertex_attributes);

    let pipeline = unsafe { context.device().create_graphics_pipeline(&desc, None)? };

    Ok(Self {
      context: context.clone(),
      pipeline: pipeline.into(),
      layout: layout.into(),
      push_constant_count,
      descriptor_layouts: builder.desriptor_layouts,
      _shaders: builder.shaders,
    })
  }

  /// Returns a reference to the descriptor layouts defined in the pipeline.
  pub fn descriptor_layouts(&self) -> &[DescriptorLayout] {
    &self.descriptor_layouts
  }

  /// Returns the number of push constants defined in the pipeline.
  pub fn push_constant_count(&self) -> usize {
    self.push_constant_count
  }

  /// Returns a reference to the underlying backend pipeline layout.
  pub fn layout(&self) -> &backend::PipelineLayout {
    &self.layout
  }

  /// Returns a reference to the underlying backend pipeline.
  pub fn as_backend(&self) -> &backend::Pipeline {
    &self.pipeline
  }
}

impl Drop for Graphics {
  fn drop(&mut self) {
    let device = self.context.device();

    unsafe {
      device.destroy_graphics_pipeline(self.pipeline.take());
      device.destroy_pipeline_layout(self.layout.take());
    }
  }
}

#[derive(Debug)]
pub enum GraphicsError {
  /// No render pass was provided.
  NoRenderPass,
  /// No vertex shader was provided.
  NoVertexShader,
  /// Some other creation error occurred.
  CreationFailed(CreationError),
}

impl std::error::Error for GraphicsError {}

impl fmt::Display for GraphicsError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      GraphicsError::NoRenderPass => write!(f, "a render pass is required"),
      GraphicsError::NoVertexShader => write!(f, "a vertex shader is required"),
      GraphicsError::CreationFailed(cause) => write!(f, "{}", cause),
    }
  }
}

// Impl `From` to convert from backend creation errors.
impl From<CreationError> for GraphicsError {
  fn from(cause: CreationError) -> Self {
    GraphicsError::CreationFailed(cause)
  }
}
