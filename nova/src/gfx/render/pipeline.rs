// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
pub use gfx_hal::pso::PipelineStage;

/// Represents a graphics pipeline object on the device.
#[derive(Clone)]
pub struct Pipeline(Arc<PipelineInner>);

struct PipelineInner {
  context: Context,
  pipeline: Option<backend::Pipeline>,
  layout: Option<backend::PipelineLayout>,
  push_constant_count: usize,
  descriptor_layouts: Vec<DescriptorLayout>,
  _shaders: ShaderSet,
}

impl Pipeline {
  /// Creates a new graphics pipeline object from the given `PipelineBuilder`.
  pub fn new(context: &Context, builder: PipelineBuilder) -> Result<Self, PipelineCreationError> {
    debug_assert!(
      builder.size_of_push_constants % 4 == 0,
      "size_of_push_constants must be a multiple of 4"
    );

    let push_constant_count = builder.size_of_push_constants / 4;

    let render_pass = builder
      .render_pass
      .as_ref()
      .ok_or(PipelineCreationError::NoRenderPass)?;

    let descriptor_layouts = builder
      .desriptor_layouts
      .iter()
      .map(DescriptorLayout::as_backend);

    let push_constant_ranges = if builder.size_of_push_constants > 0 {
      Some((
        gfx_hal::pso::ShaderStageFlags::ALL,
        0..push_constant_count as u32,
      ))
    } else {
      None
    };

    let layout = unsafe {
      context
        .device
        .create_pipeline_layout(descriptor_layouts, push_constant_ranges)?
    };

    let mut desc = gfx_hal::pso::GraphicsPipelineDesc::new(
      gfx_hal::pso::GraphicsShaderSet {
        vertex: builder
          .shaders
          .vertex
          .as_ref()
          .ok_or(PipelineCreationError::NoVertexShader)?
          .backend_entrypoint(),

        fragment: builder
          .shaders
          .fragment
          .as_ref()
          .map(shader::Module::backend_entrypoint),

        geometry: None,
        domain: None,
        hull: None,
      },
      gfx_hal::Primitive::TriangleStrip,
      gfx_hal::pso::Rasterizer::FILL,
      &layout,
      gfx_hal::pass::Subpass {
        index: 0,
        main_pass: render_pass.as_backend(),
      },
    );

    desc.blender.targets.push(gfx_hal::pso::ColorBlendDesc(
      gfx_hal::pso::ColorMask::ALL,
      gfx_hal::pso::BlendState::ALPHA,
    ));

    desc.vertex_buffers.extend(builder.vertex_buffers);
    desc.attributes.extend(builder.vertex_attributes);

    let pipeline = unsafe { context.device.create_graphics_pipeline(&desc, None)? };

    Ok(Self(Arc::new(PipelineInner {
      context: context.clone(),
      pipeline: Some(pipeline),
      layout: Some(layout),
      push_constant_count,
      descriptor_layouts: builder.desriptor_layouts,
      _shaders: builder.shaders,
    })))
  }

  /// Returns a reference to the descriptor layouts defined in the pipeline.
  pub fn descriptor_layouts(&self) -> &[DescriptorLayout] {
    &self.0.descriptor_layouts
  }

  /// Returns the number of push constants defined in the pipeline.
  pub fn push_constant_count(&self) -> usize {
    self.0.push_constant_count
  }

  /// Returns a reference to the underlying backend pipeline.
  pub(crate) fn as_backend(&self) -> &backend::Pipeline {
    self.0.pipeline.as_ref().unwrap()
  }

  /// Returns a reference to the underlying backend pipeline layout.
  pub(crate) fn as_backend_layout(&self) -> &backend::PipelineLayout {
    self.0.layout.as_ref().unwrap()
  }
}

impl Drop for PipelineInner {
  fn drop(&mut self) {
    unsafe {
      self
        .context
        .device
        .destroy_graphics_pipeline(self.pipeline.take().unwrap());

      self
        .context
        .device
        .destroy_pipeline_layout(self.layout.take().unwrap());
    }
  }
}

/// Container for all of the possible shaders in a pipeline.
#[derive(Clone, Default)]
struct ShaderSet {
  pub vertex: Option<shader::Module>,
  pub fragment: Option<shader::Module>,
}

/// A declarative builder for creating a `Pipeline`.
#[derive(Default)]
pub struct PipelineBuilder {
  shaders: ShaderSet,
  size_of_push_constants: usize,
  render_pass: Option<Pass>,
  vertex_buffers: Vec<gfx_hal::pso::VertexBufferDesc>,
  vertex_attributes: Vec<gfx_hal::pso::AttributeDesc>,
  desriptor_layouts: Vec<DescriptorLayout>,
}

impl PipelineBuilder {
  /// Creates a new builder.
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the render pass of the pipeline.
  pub fn set_render_pass(mut self, render_pass: &Pass) -> Self {
    self.render_pass = Some(render_pass.clone());
    self
  }

  /// Sets the vertex shader of the pipeline.
  pub fn set_vertex_shader(mut self, module: &shader::Module) -> Self {
    self.shaders.vertex = Some(module.clone());
    self
  }

  /// Sets the fragment shader of the pipeline.
  pub fn set_fragment_shader(mut self, module: &shader::Module) -> Self {
    self.shaders.fragment = Some(module.clone());
    self
  }

  /// Sets the push constants type of the pipeline to `T`.
  pub fn set_push_constants<T: Sized>(mut self) -> Self {
    self.size_of_push_constants = mem::size_of::<T>();
    self
  }

  /// Adds a vertex buffer of type `T` to the pipeline.
  pub fn add_vertex_buffer<T: vertex::Data>(mut self) -> Self {
    let binding = self.vertex_buffers.len() as u32;

    self.vertex_buffers.push(gfx_hal::pso::VertexBufferDesc {
      binding,
      stride: T::stride(),
      rate: gfx_hal::pso::VertexInputRate::Vertex,
    });

    let mut offset = 0;

    for attribute in T::ATTRIBUTES {
      self.vertex_attributes.push(gfx_hal::pso::AttributeDesc {
        binding,
        location: self.vertex_attributes.len() as u32,
        element: gfx_hal::pso::Element {
          format: attribute.backend_format(),
          offset,
        },
      });

      offset += attribute.size();
    }

    self
  }

  /// Adds a [`DescriptorSet`] to the pipeline with the given `layout`.
  pub fn add_descriptor_set(mut self, layout: &DescriptorLayout) -> Self {
    self.desriptor_layouts.push(layout.clone());
    self
  }

  /// Builds the [`Pipeline`].
  pub fn build(self, context: &Context) -> Result<Pipeline, PipelineCreationError> {
    Pipeline::new(context, self)
  }
}

#[derive(Debug)]
pub enum PipelineCreationError {
  /// No render pass was provided.
  NoRenderPass,
  /// No vertex shader was provided.
  NoVertexShader,
  /// The specified shader stage is not supported.
  UnsupportedShaderStage(shader::Stage),
  /// A shader had an incorrect interface, such as the wrong number of push
  /// constants.
  ShaderInterfaceMismatch(String),
  /// A shader is missing a `main` function.
  ShaderMissingEntryPoint(String),
  /// Invalid subpass.
  InvalidSubpass(usize),
  /// Out of either host or device memory.
  OutOfMemory,
}

impl From<gfx_hal::pso::CreationError> for PipelineCreationError {
  fn from(err: gfx_hal::pso::CreationError) -> Self {
    match err {
      gfx_hal::pso::CreationError::InvalidSubpass(id) => PipelineCreationError::InvalidSubpass(id),
      gfx_hal::pso::CreationError::OutOfMemory(_) => PipelineCreationError::OutOfMemory,
      gfx_hal::pso::CreationError::Shader(err) => match err {
        gfx_hal::device::ShaderError::UnsupportedStage(stage) => {
          PipelineCreationError::UnsupportedShaderStage(stage)
        }
        gfx_hal::device::ShaderError::InterfaceMismatch(reason) => {
          PipelineCreationError::ShaderInterfaceMismatch(reason)
        }
        gfx_hal::device::ShaderError::MissingEntryPoint(reason) => {
          PipelineCreationError::ShaderMissingEntryPoint(reason)
        }
        err => panic!("failed to create pipeline: {}", err),
      },
      err => panic!("failed to create pipeline: {}", err),
    }
  }
}

impl From<gfx_hal::device::OutOfMemory> for PipelineCreationError {
  fn from(_: gfx_hal::device::OutOfMemory) -> Self {
    PipelineCreationError::OutOfMemory
  }
}

impl fmt::Display for PipelineCreationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      PipelineCreationError::NoRenderPass => write!(f, "no render pass"),
      PipelineCreationError::NoVertexShader => write!(f, "no vertex shader provided"),
      PipelineCreationError::UnsupportedShaderStage(stage) => {
        write!(f, "shader stage {:?} is not supported", stage)
      }
      PipelineCreationError::ShaderInterfaceMismatch(err) => {
        write!(f, "shader interface mismatch: {}", err)
      }
      PipelineCreationError::ShaderMissingEntryPoint(err) => {
        write!(f, "shader missing entry point: {}", err)
      }
      PipelineCreationError::InvalidSubpass(id) => write!(f, "invalid subpass: {}", id),
      PipelineCreationError::OutOfMemory => write!(f, "out of memory"),
    }
  }
}
