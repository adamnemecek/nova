use super::buffer::{self, Buffer};
use super::vertices::*;
use super::{Color, Device};
use nova::math::algebra::Vector2;
use std::sync::Arc;

pub struct Mesh {
  indices: u32,
  vertex_buffer: Buffer<Vertex>,
  index_buffer: Buffer<u16>,
}

impl Mesh {
  pub fn new(device: &Arc<Device>, vertices: &[Vertex], indices: &[u16]) -> Mesh {
    assert!(vertices.len() > 0, "mesh has no vertices");
    assert!(indices.len() > 0, "mesh has no indices");

    let mut vertex_buffer = Buffer::new(device, vertices.len(), buffer::Usage::VERTEX);

    vertex_buffer.write(&vertices);

    let mut index_buffer = Buffer::new(device, indices.len(), buffer::Usage::INDEX);

    index_buffer.write(&indices);

    Mesh {
      indices: indices.len() as u32,
      vertex_buffer,
      index_buffer,
    }
  }

  pub fn index_count(&self) -> u32 {
    self.indices
  }

  pub fn vertex_buffer(&self) -> &Buffer<Vertex> {
    &self.vertex_buffer
  }

  pub fn index_buffer(&self) -> &Buffer<u16> {
    &self.index_buffer
  }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vertex {
  pub pos: Vector2<f32>,
  pub color: Color,
  pub tex_pos: Vector2<f32>,
}

impl Vertex {
  pub fn new(pos: [f32; 2], color: [f32; 4], tex_pos: [f32; 2]) -> Self {
    Vertex {
      pos: Vector2::new(pos[0], pos[1]),
      color: Color(color),
      tex_pos: Vector2::new(tex_pos[0], tex_pos[1]),
    }
  }
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
