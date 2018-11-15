// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{DescriptorLayout, Pipeline, ShaderSet, VertexData};
use crate::graphics::hal::prelude::*;
use crate::graphics::{Device, RenderPass};
use std::ops::Range;
use std::sync::Arc;

/// Builds a new [`Pipeline`].
#[derive(Default)]
pub struct PipelineBuilder {
  render_pass: Option<Arc<RenderPass>>,
  shaders: Option<ShaderSet>,
  vertex_buffers: Vec<hal::pso::VertexBufferDesc>,
  vertex_attributes: Vec<hal::pso::AttributeDesc>,
  push_constants: Vec<(hal::pso::ShaderStageFlags, Range<u32>)>,
  descriptor_layouts: Vec<Arc<DescriptorLayout>>,
}

impl PipelineBuilder {
  /// Sets the render pass of the pipeline.
  pub fn render_pass(mut self, pass: &Arc<RenderPass>) -> Self {
    self.render_pass = Some(pass.clone());
    self
  }

  /// Adds a vertex buffer to the pipeline.
  ///
  /// Buffers are bound in the order they are added starting from index 0.
  pub fn vertex_buffer<T: VertexData>(mut self) -> Self {
    let binding = self.vertex_buffers.len() as u32;

    self.vertex_buffers.push(hal::pso::VertexBufferDesc {
      binding,
      stride: T::stride(),
      rate: 0,
    });

    let mut offset = 0;

    self
      .vertex_attributes
      .extend(T::attributes().iter().enumerate().map(|(i, attr)| {
        let desc = hal::pso::AttributeDesc {
          location: i as u32,
          binding,
          element: hal::pso::Element {
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
  pub fn push_constant<T>(mut self) -> Self {
    let size = std::mem::size_of::<T>();

    assert!(
      size % 4 == 0,
      "Push constants must be a multiple of 4 bytes in size."
    );

    let start = self
      .push_constants
      .last()
      .map(|(_, range)| range.end)
      .unwrap_or(0);

    let end = start + size as u32 / 4;

    assert!(
      end <= 32,
      "Push constants should not exceed 128 bytes total."
    );

    self.push_constants.push((
      hal::pso::ShaderStageFlags::VERTEX | hal::pso::ShaderStageFlags::FRAGMENT,
      start..end,
    ));

    self
  }

  /// Adds a descriptor layout to the pipeline.
  ///
  /// Descriptor layouts are bound in the order they are added starting from
  /// index 0.
  pub fn descriptor_layout(mut self, layout: &Arc<DescriptorLayout>) -> Self {
    self.descriptor_layouts.push(layout.clone());
    self
  }

  /// Sets the shaders the pipeline will use.
  pub fn shaders(mut self, shaders: ShaderSet) -> Self {
    self.shaders = Some(shaders);
    self
  }

  /// Builds the pipeline for the given device.
  pub fn build(self, device: &Arc<Device>) -> Arc<Pipeline> {
    let render_pass = self.render_pass.expect("A render pass is required.");
    let shaders = self.shaders.expect("A shader set is required.");

    let subpass = hal::pass::Subpass {
      index: 0,
      main_pass: render_pass.raw(),
    };

    let layout = device
      .raw()
      .create_pipeline_layout(
        self
          .descriptor_layouts
          .iter()
          .map(AsRef::as_ref)
          .map(AsRef::as_ref),
        &self.push_constants,
      )
      .expect("Could not create pipeline layout");

    let mut pipeline_desc = hal::pso::GraphicsPipelineDesc::new(
      (&shaders).into(),
      hal::Primitive::TriangleList,
      hal::pso::Rasterizer::FILL,
      &layout,
      subpass,
    );

    pipeline_desc.blender.targets.push(hal::pso::ColorBlendDesc(
      hal::pso::ColorMask::ALL,
      hal::pso::BlendState::ALPHA,
    ));

    pipeline_desc
      .vertex_buffers
      .extend(self.vertex_buffers.into_iter());

    pipeline_desc
      .attributes
      .extend(self.vertex_attributes.into_iter());

    let pipeline = device
      .raw()
      .create_graphics_pipeline(&pipeline_desc, None)
      .expect("could not create graphics pipeline");

    Arc::new(Pipeline {
      device: device.clone(),
      raw: (pipeline, layout).into(),
      push_constants: self.push_constants,
      descriptor_layouts: self.descriptor_layouts,
      shaders,
    })
  }
}
