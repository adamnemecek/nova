// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::pso::PipelineStage as Stage;

use crate::{backend, renderer, shader, Context};
use gfx_hal::Device as _;
use std::{fmt, iter};
use std::sync::Arc;

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
  pub fn new(
    context: &Context,
    render_pass: &renderer::RenderPass,
    options: Options,
  ) -> Result<Self, CreationError> {
    debug_assert!(
      options.size_of_push_constants % 4 == 0,
      "size_of_push_constants must be a multiple of 4"
    );

    let push_constant_count = options.size_of_push_constants / 4;

    let layout = unsafe {
      context.device.create_pipeline_layout(
        iter::empty::<backend::DescriptorLayout>(),
        if options.size_of_push_constants > 0 {
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
        vertex: options.shaders.vertex.backend_entrypoint(),
        fragment: Some(options.shaders.fragment.backend_entrypoint()),
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

    let pipeline = unsafe { context.device.create_graphics_pipeline(&desc, None)? };

    Ok(Self (Arc::new(GraphicsInner{
      context: context.clone(),
      pipeline: Some(pipeline),
      layout: Some(layout),
      push_constant_count,
      _shaders: options.shaders.clone(),
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

#[derive(Clone)]
pub struct ShaderSet {
  pub vertex: shader::Module,
  pub fragment: shader::Module,
}

pub struct Options {
  pub shaders: ShaderSet,
  pub size_of_push_constants: usize,
}

#[derive(Debug)]
pub enum CreationError {
  /// The specified shader stage is not supported.y
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
