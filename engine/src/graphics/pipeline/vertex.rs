// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::image;
use std::mem;

/// A trait that allows a type to be used as a vertex in a vertex buffer.
pub trait VertexData: Sized {
  /// Gets the vertex stride, which is usually the size of the vertex data
  /// structure.
  fn stride() -> u32 {
    mem::size_of::<Self>() as u32
  }

  /// Gets a list of the vertex's attributes, the individual fields accessible
  /// in the vertex shader.
  fn attributes() -> &'static [VertexAttribute];
}

/// One of the kinds of vertex attribute, the individual fields accessible in
/// a vertex shader.
pub enum VertexAttribute {
  Vector2f32,
  Vector4f32,
}

impl VertexAttribute {
  /// Gets the size of the vertex attribute in bytes.
  pub fn size(&self) -> u32 {
    match self {
      VertexAttribute::Vector2f32 => 8,
      VertexAttribute::Vector4f32 => 16,
    }
  }
}

// Implement `From` to convert attributes to the equivalent raw image format.
impl From<&VertexAttribute> for image::Format {
  fn from(attr: &VertexAttribute) -> Self {
    match attr {
      VertexAttribute::Vector2f32 => image::Format::Rg32Float,
      VertexAttribute::Vector4f32 => image::Format::Rgba32Float,
    }
  }
}
