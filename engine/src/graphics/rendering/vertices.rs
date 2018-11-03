use std::mem;

pub trait VertexData: Sized {
  fn stride() -> u32 {
    mem::size_of::<Self>() as u32
  }

  fn attributes() -> &'static [VertexAttribute];
}

pub enum VertexAttribute {
  Vector2f32,
  Vector4f32,
}

impl VertexAttribute {
  pub fn size(&self) -> u32 {
    match self {
      VertexAttribute::Vector2f32 => 8,
      VertexAttribute::Vector4f32 => 16,
    }
  }
}

impl From<&VertexAttribute> for gfx_hal::format::Format {
  fn from(attr: &VertexAttribute) -> Self {
    match attr {
      VertexAttribute::Vector2f32 => gfx_hal::format::Format::Rg32Float,
      VertexAttribute::Vector4f32 => gfx_hal::format::Format::Rgba32Float,
    }
  }
}
