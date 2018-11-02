use super::buffers::Buffer;
use super::{Backend, Context};
use nalgebra::{Vector2, Vector4};
use std::sync::Arc;

pub struct Mesh {
  vertices: Vec<Vertex>,
  vertex_buffer: Buffer<Vertex>,
}

impl Mesh {
  pub fn new(context: &Arc<Context>, vertices: Vec<Vertex>) -> Mesh {
    let vertex_buffer = Buffer::new(context, vertices.len());

    vertex_buffer.write(&vertices);

    Mesh {
      vertices,
      vertex_buffer,
    }
  }

  pub fn vertices(&self) -> &[Vertex] {
    &self.vertices
  }

  pub fn vertex_buffer(&self) -> &Buffer<Vertex> {
    &self.vertex_buffer
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
}
