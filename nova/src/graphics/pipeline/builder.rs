// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Inner, Pipeline, Shader, ShaderSet, VertexData};
use crate::graphics::device::RawDeviceExt;
use crate::graphics::renderer;
use crate::utils::quick_error;
use std::ops::Range;
use std::sync::Arc;

/// Builds a new [`Pipeline`].
#[derive(Default)]
pub struct PipelineBuilder {
  render_pass: Option<renderer::Pass>,
  vertex_shader: Option<Shader>,
  fragment_shader: Option<Shader>,
  vertex_buffers: Vec<gfx_hal::pso::VertexBufferDesc>,
  vertex_attributes: Vec<gfx_hal::pso::AttributeDesc>,
  push_constants: Vec<Range<u32>>,
}

impl PipelineBuilder {
  /// Creates a new builder.
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the render pass of the pipeline.
  pub fn set_render_pass(mut self, pass: &renderer::Pass) -> Self {
    self.render_pass = Some(pass.clone());
    self
  }

  /// Sets the vertex shader the pipeline will use.
  pub fn set_vertex_shader(mut self, shader: &Shader) -> Self {
    self.vertex_shader = Some(shader.clone());
    self
  }

  /// Sets the fragment shader the pipeline will use.
  pub fn set_fragment_shader(mut self, shader: &Shader) -> Self {
    self.fragment_shader = Some(shader.clone());
    self
  }

  /// Adds a vertex buffer to the pipeline.
  ///
  /// Buffers are bound in the order they are added starting from index 0.
  pub fn add_vertex_buffer<T: VertexData>(mut self) -> Self {
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

  /// Adds a push constant to the pipeline.
  ///
  /// Push constants are bound in the order they are added starting from index
  /// 0.
  pub fn add_push_constant<T>(mut self) -> Self {
    let size = std::mem::size_of::<T>();

    assert!(
      size % 4 == 0,
      "Push constants must be a multiple of 4 bytes in size."
    );

    let start = self.push_constants.last().map(|r| r.end).unwrap_or(0);
    let end = start + size as u32 / 4;

    assert!(
      end <= 32,
      "Push constants should not exceed 128 bytes total."
    );

    self.push_constants.push(start..end);

    self
  }

  /// Builds the pipeline for the given device.
  pub fn build(self) -> Result<Pipeline, BuildError> {
    let render_pass = self.render_pass.ok_or(BuildError::RenderPassRequired)?;

    let shader_set = gfx_hal::pso::GraphicsShaderSet {
      domain: None,
      fragment: self
        .fragment_shader
        .as_ref()
        .map(|frag| gfx_hal::pso::EntryPoint {
          module: frag.raw(),
          entry: "main",
          specialization: Default::default(),
        }),
      geometry: None,
      hull: None,
      vertex: gfx_hal::pso::EntryPoint {
        module: self
          .vertex_shader
          .as_ref()
          .ok_or(BuildError::VertexShaderRequired)?
          .raw(),
        entry: "main",
        specialization: Default::default(),
      },
    };

    let device = render_pass.device();

    let subpass = gfx_hal::pass::Subpass {
      index: 0,
      main_pass: render_pass.raw(),
    };

    let push_constants = if self.push_constants.is_empty() {
      Vec::new()
    } else {
      vec![(
        gfx_hal::pso::ShaderStageFlags::VERTEX | gfx_hal::pso::ShaderStageFlags::FRAGMENT,
        self.push_constants.first().unwrap().start..self.push_constants.last().unwrap().end,
      )]
    };

    let layout = unsafe { device.raw().create_pipeline_layout(&[], &push_constants)? };

    let mut pipeline_desc = gfx_hal::pso::GraphicsPipelineDesc::new(
      shader_set,
      gfx_hal::Primitive::TriangleStrip,
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
      .extend(self.vertex_buffers.into_iter());

    pipeline_desc
      .attributes
      .extend(self.vertex_attributes.into_iter());

    let pipeline = unsafe {
      device
        .raw()
        .create_graphics_pipeline(&pipeline_desc, None)?
    };

    Ok(Pipeline {
      inner: Arc::new(Inner {
        device: device.clone(),
        raw: pipeline.into(),
        raw_layout: layout.into(),
        push_constants: self.push_constants,
        _shaders: ShaderSet {
          vertex: self.vertex_shader.unwrap(),
          fragment: self.fragment_shader,
        },
      }),
    })
  }
}

quick_error! {
  #[derive(Debug)]
  pub enum BuildError {
    RenderPassRequired {
      display("a render pass must be provided with `set_render_pass()`.")
    }
    VertexShaderRequired {
      display("a vertex shader must be provided with `set_vertex_shader()`.")
    }
    OutOfMemory(inner: gfx_hal::device::OutOfMemory) {
      display("out of memory.")
      from()
    }
    CreationError(err: gfx_hal::pso::CreationError) {
      display("could not create backend pipeline object: {}.", err)
      from()
    }
  }
}
