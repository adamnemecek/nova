use super::backend;
use super::Renderer;
use crate::prelude::*;
use gfx_hal as hal;
use gfx_hal::Device;

#[repr(C)]
pub struct Vertex {
  pub pos: Vector2<f32>,
  pub color: Vector4<f32>,
}

impl Vertex {}

pub struct Mesh {}

impl Mesh {
  pub fn new(renderer: &Renderer) {
    let unbound: backend::UnboundBuffer = renderer
      .device
      .create_buffer(128, hal::buffer::Usage::VERTEX)
      .unwrap();

    let required = renderer.device.get_buffer_requirements(&unbound);

    let memory = renderer
      .device
      .allocate_memory(upload_type, required.size)
      .unwrap();

    let buffer = renderer
      .device
      .bind_buffer_memory(&memory, 0, unbound)
      .unwrap();
  }
}
