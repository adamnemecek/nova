// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::image;
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

impl From<&VertexAttribute> for image::Format {
  fn from(attr: &VertexAttribute) -> Self {
    match attr {
      VertexAttribute::Vector2f32 => image::Format::Rg32Float,
      VertexAttribute::Vector4f32 => image::Format::Rgba32Float,
    }
  }
}
