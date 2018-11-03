use super::buffers::{Buffer, BufferUsage};
use super::{Backend, Context};
use nalgebra::{Vector2, Vector4};
use std::sync::Arc;

pub struct Mesh {
  indices: u32,
  vertex_buffer: Buffer<Vertex>,
  index_buffer: Buffer<u16>,
}

impl Mesh {
  pub fn new(context: &Arc<Context>, vertices: Vec<Vertex>, indices: Vec<u16>) -> Mesh {
    let vertex_buffer = Buffer::new(context, vertices.len(), BufferUsage::VERTEX);
    let index_buffer = Buffer::new(context, indices.len(), BufferUsage::INDEX);

    vertex_buffer.write(&vertices);
    index_buffer.write(&indices);

    Mesh {
      indices: indices.len() as u32,
      vertex_buffer,
      index_buffer,
    }
  }

  pub fn indices(&self) -> u32 {
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
  pub color: Vector4<f32>,
}

impl Vertex {
  pub fn add_pipeline_binding(
    pipeline_desc: &mut gfx_hal::pso::GraphicsPipelineDesc<Backend>,
    binding: u32,
  ) {
    pipeline_desc
      .vertex_buffers
      .push(gfx_hal::pso::VertexBufferDesc {
        binding,
        stride: std::mem::size_of::<Self>() as u32,
        rate: 0,
      });

    pipeline_desc.attributes.push(gfx_hal::pso::AttributeDesc {
      binding,
      location: 0,
      element: gfx_hal::pso::Element {
        format: gfx_hal::format::Format::Rg32Float,
        offset: 0,
      },
    });

    pipeline_desc.attributes.push(gfx_hal::pso::AttributeDesc {
      binding,
      location: 1,
      element: gfx_hal::pso::Element {
        format: gfx_hal::format::Format::Rgb32Float,
        offset: 8,
      },
    });
  }

  pub fn new(pos: [f32; 2], color: [f32; 4]) -> Self {
    Vertex {
      pos: Vector2::new(pos[0], pos[1]),
      color: Vector4::new(color[0], color[1], color[2], color[3]),
    }
  }
}
