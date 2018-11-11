// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::pso::PipelineStage as Stage;

use super::{RenderPass, Shader, VertexData};
use crate::graphics::backend;
use crate::graphics::hal::prelude::*;
use crate::graphics::image::{self, Image};
use crate::graphics::Device;
use std::iter;
use std::ops::Range;
use std::sync::{Arc, Mutex};

pub struct Pipeline {
  device: Arc<Device>,
  raw: Option<backend::GraphicsPipeline>,
  layout: Option<backend::PipelineLayout>,
  push_constants: Vec<(hal::pso::ShaderStageFlags, Range<u32>)>,
  descriptor_set_layout: Option<Arc<DescriptorSetLayout>>,
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

  pub fn descriptor_set_layout(&self) -> Option<&Arc<DescriptorSetLayout>> {
    self.descriptor_set_layout.as_ref()
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
  descriptor_set_layout: Option<Arc<DescriptorSetLayout>>,
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

  pub fn descriptor_set_layout(mut self, layout: &Arc<DescriptorSetLayout>) -> Self {
    self.descriptor_set_layout = Some(layout.clone());
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
          .descriptor_set_layout
          .as_ref()
          .map(|layout| layout.raw()),
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
      descriptor_set_layout: self.descriptor_set_layout,
      _shaders: self.shaders,
    })
  }
}

pub struct DescriptorSetLayout {
  bindings: Vec<hal::pso::DescriptorSetLayoutBinding>,
  raw: Option<backend::DescriptorSetLayout>,
  device: Arc<Device>,
}

impl DescriptorSetLayout {
  pub fn new() -> DescriptorSetLayoutBuilder {
    DescriptorSetLayoutBuilder::default()
  }

  pub fn bindings(&self) -> impl Iterator<Item = &hal::pso::DescriptorSetLayoutBinding> {
    self.bindings.iter()
  }

  pub fn raw(&self) -> &backend::DescriptorSetLayout {
    self.raw.as_ref().unwrap()
  }
}

impl Drop for DescriptorSetLayout {
  fn drop(&mut self) {
    if let Some(layout) = self.raw.take() {
      self.device.raw().destroy_descriptor_set_layout(layout);
    }
  }
}

#[derive(Default)]
pub struct DescriptorSetLayoutBuilder {
  bindings: Vec<hal::pso::DescriptorSetLayoutBinding>,
}

impl DescriptorSetLayoutBuilder {
  pub fn texture(mut self) -> Self {
    let binding = self.bindings.len() as u32;

    self.bindings.push(hal::pso::DescriptorSetLayoutBinding {
      binding,
      ty: hal::pso::DescriptorType::SampledImage,
      count: 1,
      stage_flags: hal::pso::ShaderStageFlags::FRAGMENT,
      immutable_samplers: false,
    });

    self
  }

  pub fn build(self, device: &Arc<Device>) -> Arc<DescriptorSetLayout> {
    let layout = device
      .raw()
      .create_descriptor_set_layout(&self.bindings, &[])
      .expect("could not create descriptor set layout");

    Arc::new(DescriptorSetLayout {
      raw: Some(layout),
      device: device.clone(),
      bindings: self.bindings,
    })
  }
}

pub struct DescriptorPool {
  device: Arc<Device>,
  layout: Arc<DescriptorSetLayout>,
  raw: Mutex<Option<backend::DescriptorPool>>,
}

impl DescriptorPool {
  pub fn new(layout: &Arc<DescriptorSetLayout>, capacity: usize) -> Arc<Self> {
    let device = layout.device.clone();

    let pool = device
      .raw()
      .create_descriptor_pool(
        capacity,
        layout
          .bindings()
          .map(|binding| hal::pso::DescriptorRangeDesc {
            ty: binding.ty,
            count: binding.count,
          }),
      )
      .expect("could not create descriptor pool");

    Arc::new(DescriptorPool {
      device,
      layout: layout.clone(),
      raw: Mutex::new(Some(pool)),
    })
  }
}

impl Drop for DescriptorPool {
  fn drop(&mut self) {
    if let Some(pool) = self.raw.lock().unwrap().take() {
      self.device.raw().destroy_descriptor_pool(pool);
    }
  }
}

pub enum Descriptor<'a> {
  Texture(&'a Image, &'a image::Sampler),
}

pub struct DescriptorSet {
  pool: Arc<DescriptorPool>,
  raw: Option<backend::DescriptorSet>,
}

impl DescriptorSet {
  pub fn new(pool: &Arc<DescriptorPool>, descriptors: &[Descriptor]) -> DescriptorSet {
    let device = pool.device.raw();

    let mut raw_pool = pool.raw.lock().unwrap();

    let set = raw_pool
      .as_mut()
      .unwrap()
      .allocate_set(pool.layout.raw())
      .expect("could not allocate descriptor set");

    device.write_descriptor_sets(descriptors.iter().enumerate().map(|(i, descriptor)| {
      hal::pso::DescriptorSetWrite {
        set: &set,
        binding: i as u32,
        array_offset: 0,
        descriptors: iter::once(match descriptor {
          Descriptor::Texture(image, sampler) => hal::pso::Descriptor::CombinedImageSampler(
            image.as_ref(),
            hal::image::Layout::ShaderReadOnlyOptimal,
            sampler.raw(),
          ),
        }),
      }
    }));

    DescriptorSet {
      pool: pool.clone(),
      raw: Some(set),
    }
  }

  pub fn raw(&self) -> &backend::DescriptorSet {
    self.raw.as_ref().unwrap()
  }
}

impl Drop for DescriptorSet {
  fn drop(&mut self) {
    if let Some(set) = self.raw.take() {
      self
        .pool
        .raw
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .free_sets(iter::once(set));
    }
  }
}
