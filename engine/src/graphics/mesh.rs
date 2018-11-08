use super::device::{self, Device};
use super::rendering;
use super::Color;
use nalgebra::Vector2;
use std::sync::Arc;

pub struct Mesh {
  vertices: u32,
  indices: u32,
  vertex_buffer: device::Buffer<Vertex>,
  index_buffer: device::Buffer<u16>,
}

impl Mesh {
  pub fn new(device: &Arc<Device>, vertices: &[Vertex], indices: &[u16]) -> Mesh {
    assert!(vertices.len() > 0, "mesh has no vertices");
    assert!(indices.len() > 0, "mesh has no indices");

    let mut vertex_buffer =
      device::Buffer::new(device, vertices.len(), device::BufferUsage::VERTEX);

    vertex_buffer.write(&vertices);

    let mut index_buffer = device::Buffer::new(device, indices.len(), device::BufferUsage::INDEX);

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

  pub fn vertex_buffer(&self) -> &device::Buffer<Vertex> {
    &self.vertex_buffer
  }

  pub fn index_buffer(&self) -> &device::Buffer<u16> {
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

impl rendering::VertexData for Vertex {
  fn attributes() -> &'static [rendering::VertexAttribute] {
    &[
      rendering::VertexAttribute::Vector2f32,
      rendering::VertexAttribute::Vector4f32,
      rendering::VertexAttribute::Vector2f32,
    ]
  }
}
