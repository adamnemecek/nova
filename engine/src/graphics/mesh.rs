use super::rendering;
use nalgebra::{Vector2, Vector4};
use std::sync::Arc;

pub struct Mesh {
  vertices: u32,
  indices: u32,
  vertex_buffer: rendering::Buffer<Vertex>,
  index_buffer: rendering::Buffer<u16>,
}

impl Mesh {
  pub fn new(device: &Arc<rendering::Device>, vertices: &[Vertex], indices: &[u16]) -> Mesh {
    assert!(vertices.len() > 0, "mesh has no vertices");
    assert!(indices.len() > 0, "mesh has no indices");

    let vertex_buffer =
      rendering::Buffer::new(device, vertices.len(), rendering::BufferUsage::VERTEX);

    vertex_buffer.write(&vertices);

    let index_buffer = rendering::Buffer::new(device, indices.len(), rendering::BufferUsage::INDEX);

    index_buffer.write(&indices);

    Mesh {
      vertices: vertices.len() as u32,
      indices: indices.len() as u32,
      vertex_buffer,
      index_buffer,
    }
  }

  pub fn vertices(&self) -> u32 {
    self.vertices
  }

  pub fn indices(&self) -> u32 {
    self.indices
  }

  pub fn vertex_buffer(&self) -> &rendering::Buffer<Vertex> {
    &self.vertex_buffer
  }

  pub fn index_buffer(&self) -> &rendering::Buffer<u16> {
    &self.index_buffer
  }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vertex {
  pub pos: Vector2<f32>,
  pub color: Vector4<f32>,
}

impl Vertex {
  pub fn new(pos: [f32; 2], color: [f32; 4]) -> Self {
    Vertex {
      pos: Vector2::new(pos[0], pos[1]),
      color: Vector4::new(color[0], color[1], color[2], color[3]),
    }
  }
}

impl rendering::VertexData for Vertex {
  fn attributes() -> &'static [rendering::VertexAttribute] {
    &[
      rendering::VertexAttribute::Vector2f32,
      rendering::VertexAttribute::Vector4f32,
    ]
  }
}
