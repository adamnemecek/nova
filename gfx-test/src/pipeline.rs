use super::gfx_back;
use super::gfx_back::Backend;
use super::render_pass::RenderPass;
use super::shaders::Shaders;
use gfx_hal::{
  pass::Subpass,
  pso::{
    BlendState, ColorBlendDesc, ColorMask, EntryPoint, GraphicsPipelineDesc, GraphicsShaderSet,
    Rasterizer,
  },
  Device, Primitive,
};

pub use super::gfx_back::GraphicsPipeline;

pub fn create(
  device: &gfx_back::Device,
  shaders: &Shaders,
  render_pass: &RenderPass,
) -> GraphicsPipeline {
  // The pipeline layout defines the shape of the data you can send to a shader.
  // This includes the number of uniforms and push constants. We don't need them
  // for now.
  let pipeline_layout = device.create_pipeline_layout(&[], &[]);

  let vert_entry = EntryPoint::<Backend> {
    entry: "main",
    module: &shaders.vert,
    specialization: Default::default(),
  };

  let fs_entry = EntryPoint::<Backend> {
    entry: "main",
    module: &shaders.frag,
    specialization: Default::default(),
  };

  let shader_entries = GraphicsShaderSet {
    vertex: vert_entry,
    hull: None,
    domain: None,
    geometry: None,
    fragment: Some(fs_entry),
  };

  let subpass = Subpass {
    index: 0,
    main_pass: render_pass,
  };

  let mut pipeline_desc = GraphicsPipelineDesc::new(
    shader_entries,
    Primitive::TriangleList,
    Rasterizer::FILL,
    &pipeline_layout,
    subpass,
  );

  pipeline_desc
    .blender
    .targets
    .push(ColorBlendDesc(ColorMask::ALL, BlendState::ALPHA));

  device
    .create_graphics_pipeline(&pipeline_desc, None)
    .expect("could not create graphics pipeline")
}
