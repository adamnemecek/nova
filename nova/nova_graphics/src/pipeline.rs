// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
pub use gfx_hal::pso::PipelineStage as Stage;

#[derive(Clone)]
pub struct Graphics(Arc<GraphicsInner>);

struct GraphicsInner {
  context: Context,
  pipeline: Option<backend::Pipeline>,
  layout: Option<backend::PipelineLayout>,
  push_constant_count: usize,
  _shaders: ShaderSet,
}

impl Graphics {
  pub fn new(context: &Context, builder: Builder) -> Result<Self, CreationError> {
    debug_assert!(
      builder.size_of_push_constants % 4 == 0,
      "size_of_push_constants must be a multiple of 4"
    );

    let push_constant_count = builder.size_of_push_constants / 4;

    let render_pass = builder
      .render_pass
      .as_ref()
      .ok_or(CreationError::NoRenderPass)?;

    let layout = unsafe {
      context.device.create_pipeline_layout(
        iter::empty::<backend::DescriptorLayout>(),
        if builder.size_of_push_constants > 0 {
          Some((
            gfx_hal::pso::ShaderStageFlags::ALL,
            0..push_constant_count as u32,
          ))
        } else {
          None
        },
      )?
    };

    let mut desc = gfx_hal::pso::GraphicsPipelineDesc::new(
      gfx_hal::pso::GraphicsShaderSet {
        vertex: builder
          .shaders
          .vertex
          .as_ref()
          .ok_or(CreationError::NoVertexShader)?
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

    Ok(Self(Arc::new(GraphicsInner {
      context: context.clone(),
      pipeline: Some(pipeline),
      layout: Some(layout),
      push_constant_count,
      _shaders: builder.shaders,
    })))
  }

  pub fn push_constant_count(&self) -> usize {
    self.0.push_constant_count
  }

  pub(crate) fn as_backend(&self) -> &backend::Pipeline {
    self.0.pipeline.as_ref().unwrap()
  }

  pub(crate) fn backend_layout(&self) -> &backend::PipelineLayout {
    self.0.layout.as_ref().unwrap()
  }
}

impl Drop for GraphicsInner {
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

#[derive(Clone, Default)]
pub struct ShaderSet {
  pub vertex: Option<shader::Module>,
  pub fragment: Option<shader::Module>,
}

#[derive(Default)]
pub struct Builder {
  shaders: ShaderSet,
  size_of_push_constants: usize,
  render_pass: Option<renderer::RenderPass>,
  vertex_buffers: Vec<gfx_hal::pso::VertexBufferDesc>,
  vertex_attributes: Vec<gfx_hal::pso::AttributeDesc>,
}

impl Builder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn set_render_pass(mut self, render_pass: &renderer::RenderPass) -> Self {
    self.render_pass = Some(render_pass.clone());
    self
  }

  pub fn set_vertex_shader<'a>(mut self, module: impl Into<Option<&'a shader::Module>>) -> Self {
    self.shaders.vertex = module.into().cloned();
    self
  }

  pub fn set_fragment_shader<'a>(mut self, module: impl Into<Option<&'a shader::Module>>) -> Self {
    self.shaders.fragment = module.into().cloned();
    self
  }

  pub fn set_push_constants<T: Sized>(mut self) -> Self {
    self.size_of_push_constants = mem::size_of::<T>();
    self
  }

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

  pub fn into_graphics(self, context: &Context) -> Result<Graphics, CreationError> {
    Graphics::new(context, self)
  }
}

#[derive(Debug)]
pub enum CreationError {
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

impl From<gfx_hal::pso::CreationError> for CreationError {
  fn from(err: gfx_hal::pso::CreationError) -> Self {
    match err {
      gfx_hal::pso::CreationError::InvalidSubpass(id) => CreationError::InvalidSubpass(id),
      gfx_hal::pso::CreationError::OutOfMemory(_) => CreationError::OutOfMemory,
      gfx_hal::pso::CreationError::Shader(err) => match err {
        gfx_hal::device::ShaderError::UnsupportedStage(stage) => {
          CreationError::UnsupportedShaderStage(stage)
        }
        gfx_hal::device::ShaderError::InterfaceMismatch(reason) => {
          CreationError::ShaderInterfaceMismatch(reason)
        }
        gfx_hal::device::ShaderError::MissingEntryPoint(reason) => {
          CreationError::ShaderMissingEntryPoint(reason)
        }
        err => panic!("failed to create pipeline: {}", err),
      },
      err => panic!("failed to create pipeline: {}", err),
    }
  }
}

impl From<gfx_hal::device::OutOfMemory> for CreationError {
  fn from(_: gfx_hal::device::OutOfMemory) -> Self {
    CreationError::OutOfMemory
  }
}

impl fmt::Display for CreationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      CreationError::NoRenderPass => write!(f, "no render pass"),
      CreationError::NoVertexShader => write!(f, "no vertex shader provided"),
      CreationError::UnsupportedShaderStage(stage) => {
        write!(f, "shader stage {:?} is not supported", stage)
      }
      CreationError::ShaderInterfaceMismatch(err) => {
        write!(f, "shader interface mismatch: {}", err)
      }
      CreationError::ShaderMissingEntryPoint(err) => {
        write!(f, "shader missing entry point: {}", err)
      }
      CreationError::InvalidSubpass(id) => write!(f, "invalid subpass: {}", id),
      CreationError::OutOfMemory => write!(f, "out of memory"),
    }
  }
}
