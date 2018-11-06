use super::*;
use std::iter;
use std::ops::Range;
use std::sync::{Arc, Mutex};

pub struct Pipeline {
  push_constants: Vec<(gfx_hal::pso::ShaderStageFlags, Range<u32>)>,
  raw: Option<backend::GraphicsPipeline>,
  layout: Option<backend::PipelineLayout>,
  _shaders: PipelineShaderSet,
  device: Arc<Device>,
}

impl Pipeline {
  pub fn new() -> PipelineBuilder {
    PipelineBuilder::default()
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
  descriptor_set_layout: Option<Arc<DescriptorSetLayout>>,
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

  pub fn descriptor_set_layout(mut self, layout: &Arc<DescriptorSetLayout>) -> Self {
    self.descriptor_set_layout = Some(layout.clone());
    self
  }

  pub fn create(self) -> Arc<Pipeline> {
    let render_pass = self.render_pass.expect("render_pass is required");
    let device = render_pass.device().clone();

    let subpass = gfx_hal::pass::Subpass {
      index: 0,
      main_pass: render_pass.raw(),
    };

    let layout = device.raw.create_pipeline_layout(
      self
        .descriptor_set_layout
        .as_ref()
        .map(|layout| layout.raw()),
      &self.push_constants,
    );

    let mut pipeline_desc = gfx_hal::pso::GraphicsPipelineDesc::new(
      self.shaders.as_raw(),
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
      .extend(self.vertex_buffers.into_iter());

    pipeline_desc
      .attributes
      .extend(self.vertex_attributes.into_iter());

    let pipeline = device
      .raw
      .create_graphics_pipeline(&pipeline_desc, None)
      .expect("could not create graphics pipeline");

    Arc::new(Pipeline {
      device,
      push_constants: self.push_constants,
      _shaders: self.shaders,
      layout: Some(layout),
      raw: Some(pipeline),
    })
  }
}

pub struct DescriptorSetLayout {
  bindings: Vec<gfx_hal::pso::DescriptorSetLayoutBinding>,
  raw: Option<backend::DescriptorSetLayout>,
  device: Arc<Device>,
}

impl DescriptorSetLayout {
  pub fn new() -> DescriptorSetLayoutBuilder {
    DescriptorSetLayoutBuilder::default()
  }

  pub fn bindings(&self) -> impl Iterator<Item = &gfx_hal::pso::DescriptorSetLayoutBinding> {
    self.bindings.iter()
  }

  pub fn raw(&self) -> &backend::DescriptorSetLayout {
    self.raw.as_ref().unwrap()
  }
}

impl Drop for DescriptorSetLayout {
  fn drop(&mut self) {
    if let Some(layout) = self.raw.take() {
      self.device.raw.destroy_descriptor_set_layout(layout);
    }
  }
}

#[derive(Default)]
pub struct DescriptorSetLayoutBuilder {
  bindings: Vec<gfx_hal::pso::DescriptorSetLayoutBinding>,
}

impl DescriptorSetLayoutBuilder {
  pub fn texture(mut self) -> Self {
    let binding = self.bindings.len() as u32;

    self
      .bindings
      .push(gfx_hal::pso::DescriptorSetLayoutBinding {
        binding,
        ty: gfx_hal::pso::DescriptorType::SampledImage,
        count: 1,
        stage_flags: gfx_hal::pso::ShaderStageFlags::FRAGMENT,
        immutable_samplers: false,
      });

    self
  }

  pub fn create(self, device: &Arc<Device>) -> Arc<DescriptorSetLayout> {
    let layout = device.raw.create_descriptor_set_layout(&self.bindings, &[]);

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

    let pool = device.raw.create_descriptor_pool(
      capacity,
      layout
        .bindings()
        .map(|binding| gfx_hal::pso::DescriptorRangeDesc {
          ty: binding.ty,
          count: binding.count,
        }),
    );

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
      self.device.raw.destroy_descriptor_pool(pool);
    }
  }
}

pub enum Descriptor<'a> {
  SampledTexture(&'a Texture, &'a TextureSampler),
}

pub struct DescriptorSet {
  pool: Arc<DescriptorPool>,
  raw: Option<backend::DescriptorSet>,
}

impl DescriptorSet {
  pub fn new(pool: &Arc<DescriptorPool>, descriptors: &[Descriptor]) -> DescriptorSet {
    let device = &pool.device.raw;

    let mut raw_pool = pool.raw.lock().unwrap();

    let set = raw_pool
      .as_mut()
      .unwrap()
      .allocate_set(pool.layout.raw())
      .expect("could not allocate descriptor set");

    device.write_descriptor_sets(descriptors.iter().enumerate().map(|(i, descriptor)| {
      gfx_hal::pso::DescriptorSetWrite {
        set: &set,
        binding: i as u32,
        array_offset: 0,
        descriptors: iter::once(match descriptor {
          Descriptor::SampledTexture(texture, sampler) => {
            gfx_hal::pso::Descriptor::CombinedImageSampler(
              texture.raw_view(),
              gfx_hal::image::Layout::ShaderReadOnlyOptimal,
              sampler.raw(),
            )
          }
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
