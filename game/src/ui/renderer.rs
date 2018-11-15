use super::{Hierarchy, Node, Style};
use crate::ecs;
use crate::graphics::pipeline::{self, Pipeline, VertexAttribute, VertexData};
use crate::graphics::{self, Color};
use crate::math::{Point2, Rect};
use std::sync::Arc;

const PUSH_COLOR: usize = 0;
const PUSH_RECT: usize = 1;
const DESC_TEXTURE: usize = 0;

pub struct Renderer {
  pipeline: Arc<Pipeline>,
}

impl Renderer {
  pub fn new(render_pass: &Arc<graphics::RenderPass>) -> Self {
    let device = &render_pass.device();

    let descriptor_layout =
      pipeline::DescriptorLayout::new(device, &[pipeline::descriptor::Binding::Texture]);

    let pipeline = Pipeline::new()
      .render_pass(&render_pass)
      .push_constant::<Color>()
      .push_constant::<Rect<f32>>()
      .descriptor_layout(&descriptor_layout)
      .vertex_shader(graphics::Shader::from_glsl(
        device,
        graphics::shader::Kind::Vertex,
        include_str!("shaders/default.vert"),
      ))
      .fragment_shader(graphics::Shader::from_glsl(
        device,
        graphics::shader::Kind::Fragment,
        include_str!("shaders/default.frag"),
      ))
      .build(device);

    Renderer { pipeline }
  }

  pub fn render(
    &mut self,
    ctx: &mut ecs::Context,
    entities: impl IntoIterator<Item = ecs::Entity>,
    cmd: &mut graphics::Commands,
  ) {
    let root = match ecs::get_resource_mut::<Hierarchy>(ctx).root {
      Some(r) => r,
      None => return,
    };

    cmd.bind_pipeline(&self.pipeline);

    let styles = ecs::read_components::<Style>(ctx);

    for entity in entities.into_iter() {
      let style = match styles.get(entity) {
        Some(s) => s,
        None => continue,
      };

      cmd.push_constant(PUSH_COLOR, &style.background_color);
    }
  }
}

struct PushConstants {
  tint: Color,
  rect: Rect<f32>,
}

struct Mesh {
  indices: u32,
  vertex_buffer: graphics::Buffer<Vertex>,
  index_buffer: graphics::Buffer<u16>,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Vertex {
  pub pos: Point2<f32>,
  pub color: Color,
  pub tex_pos: Point2<f32>,
}

impl VertexData for Vertex {
  fn attributes() -> &'static [VertexAttribute] {
    &[
      VertexAttribute::Vector2f32,
      VertexAttribute::Vector4f32,
      VertexAttribute::Vector2f32,
    ]
  }
}
