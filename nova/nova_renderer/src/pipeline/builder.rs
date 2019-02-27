// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Pipeline, VertexData};
use crate::descriptors::DescriptorLayout;
use crate::device::{Device, DeviceExt};
use crate::{RenderPass, Shader, ShaderSet};
use nova_core::quick_error;
use std::ops::Range;

/// Builds a new [`Pipeline`].
#[derive(Default)]
pub struct PipelineBuilder {
  vertex_shader: Option<Shader>,
  fragment_shader: Option<Shader>,
  vertex_buffers: Vec<gfx_hal::pso::VertexBufferDesc>,
  vertex_attributes: Vec<gfx_hal::pso::AttributeDesc>,
  push_constants: Vec<Range<u32>>,
  descriptor_layouts: Vec<DescriptorLayout>,
}

impl PipelineBuilder {
  /// Creates a new builder.
  pub fn new() -> Self {
    Self::default()
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

    self.vertex_attributes
      .extend(T::ATTRIBUTES.iter().enumerate().map(|(i, attr)| {
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

  pub fn add_descriptor_layout(mut self, layout: DescriptorLayout) -> Self {
    self.descriptor_layouts.push(layout);
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
  pub fn build(self, device: &Device, render_pass: &RenderPass) -> Result<Pipeline, BuildError> {
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

    let subpass = gfx_hal::pass::Subpass {
      index: 0,
      main_pass: render_pass,
    };

    let push_constants = if self.push_constants.is_empty() {
      Vec::new()
    } else {
      vec![(
        gfx_hal::pso::ShaderStageFlags::VERTEX | gfx_hal::pso::ShaderStageFlags::FRAGMENT,
        self.push_constants.first().unwrap().start..self.push_constants.last().unwrap().end,
      )]
    };

    let raw_layout = unsafe {
      device.create_pipeline_layout(
        self.descriptor_layouts.iter().map(|layout| layout.raw()),
        &push_constants,
      )?
    };

    let mut pipeline_desc = gfx_hal::pso::GraphicsPipelineDesc::new(
      shader_set,
      gfx_hal::Primitive::TriangleStrip,
      gfx_hal::pso::Rasterizer::FILL,
      &raw_layout,
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

    let raw = unsafe { device.create_graphics_pipeline(&pipeline_desc, None)? };

    Ok(Pipeline {
      raw,
      raw_layout,
      descriptor_layouts: self.descriptor_layouts,
      push_constants: self.push_constants,
      shaders: ShaderSet {
        vertex: self.vertex_shader.unwrap(),
        fragment: self.fragment_shader,
      },
    })
  }
}

quick_error! {
  #[derive(Debug)]
  pub enum BuildError {
  VertexShaderRequired {
    display("a vertex shader must be provided with `set_vertex_shader()`")
  }
  OutOfMemory(inner: gfx_hal::device::OutOfMemory) {
    display("out of memory")
    from()
  }
  CreationError(err: gfx_hal::pso::CreationError) {
    display("could not create backend pipeline object: {}", err)
    from()
  }
  }
}
