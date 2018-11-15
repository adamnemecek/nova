// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod descriptor;
pub mod vertex;

pub use self::descriptor::{Descriptor, DescriptorLayout, DescriptorPool, DescriptorSet};
pub use self::vertex::{VertexAttribute, VertexData};
pub use gfx_hal::pso::PipelineStage as Stage;

use super::{RenderPass, Shader};
use crate::graphics::backend;
use crate::graphics::hal::prelude::*;
use crate::graphics::Device;
use std::ops::Range;
use std::sync::Arc;

pub struct Pipeline {
  device: Arc<Device>,
  raw: Option<backend::GraphicsPipeline>,
  layout: Option<backend::PipelineLayout>,
  push_constants: Vec<(hal::pso::ShaderStageFlags, Range<u32>)>,
  descriptor_layouts: Vec<Arc<DescriptorLayout>>,
  _shaders: PipelineShaderSet,
}

impl Pipeline {
  pub fn new() -> PipelineBuilder {
    PipelineBuilder::default()
  }

  pub fn layout(&self) -> &backend::PipelineLayout {
    self.layout.as_ref().expect("pipeline layout was destroyed")
  }

  pub fn push_constant(&self, index: usize) -> (hal::pso::ShaderStageFlags, Range<u32>) {
    self.push_constants[index].clone()
  }

  pub fn descriptor_layouts(&self) -> &[Arc<DescriptorLayout>] {
    &self.descriptor_layouts
  }

  pub fn raw(&self) -> &backend::GraphicsPipeline {
    self.raw.as_ref().expect("pipeline was destroyed")
  }
}

impl Drop for Pipeline {
  fn drop(&mut self) {
    let device = self.device.raw();

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
  fn as_raw<'a>(&'a self) -> hal::pso::GraphicsShaderSet<'a> {
    fn entry_point(shader: &Option<Shader>) -> Option<hal::pso::EntryPoint> {
      shader.as_ref().map(|shader| hal::pso::EntryPoint {
        entry: "main",
        module: shader.raw(),
        specialization: Default::default(),
      })
    };

    hal::pso::GraphicsShaderSet {
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
  vertex_buffers: Vec<hal::pso::VertexBufferDesc>,
  vertex_attributes: Vec<hal::pso::AttributeDesc>,
  push_constants: Vec<(hal::pso::ShaderStageFlags, Range<u32>)>,
  descriptor_layouts: Vec<Arc<DescriptorLayout>>,
}

impl PipelineBuilder {
  pub fn render_pass(mut self, pass: &Arc<RenderPass>) -> Self {
    self.render_pass = Some(pass.clone());
    self
  }

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
      .push((hal::pso::ShaderStageFlags::VERTEX, start..end));

    self
  }

  pub fn descriptor_layout(mut self, layout: &Arc<DescriptorLayout>) -> Self {
    self.descriptor_layouts.push(layout.clone());
    self
  }

  pub fn vertex_shader(mut self, shader: Shader) -> Self {
    self.shaders.vertex = Some(shader);
    self
  }

  pub fn fragment_shader(mut self, shader: Shader) -> Self {
    self.shaders.fragment = Some(shader);
    self
  }

  pub fn build(self, device: &Arc<Device>) -> Arc<Pipeline> {
    let render_pass = self.render_pass.expect("render_pass is required");

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
      .expect("could not create pipeline layout");

    let mut pipeline_desc = hal::pso::GraphicsPipelineDesc::new(
      self.shaders.as_raw(),
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
      raw: Some(pipeline),
      layout: Some(layout),
      push_constants: self.push_constants,
      descriptor_layouts: self.descriptor_layouts,
      _shaders: self.shaders,
    })
  }
}
