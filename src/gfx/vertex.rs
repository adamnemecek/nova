// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A trait for types that can be used for vertex buffers.
pub trait Data: Sized {
  /// A list of the vertex's attributes, the individual fields accessible in the
  /// vertex shader.
  const ATTRIBUTES: &'static [Attribute];

  /// Gets the vertex stride, which is usually the size of the vertex data
  /// structure.
  fn stride() -> u32 {
    mem::size_of::<Self>() as u32
  }
}

/// Type of vertex attribute.
#[derive(Clone, Copy)]
pub enum Attribute {
  /// Two 32-bit floating point values.
  Vector2f32,
  /// Three 32-bit floating point values.
  Vector3f32,
  /// Four 32-bit floating point values.
  Vector4f32,
}

impl Attribute {
  /// Gets the size of the vertex attribute in bytes.
  pub fn size(self) -> u32 {
    match self {
      Attribute::Vector2f32 => 8,
      Attribute::Vector3f32 => 12,
      Attribute::Vector4f32 => 16,
    }
  }

  /// Returns the equivalent backend format.
  pub fn backend_format(self) -> gfx_hal::format::Format {
    match self {
      Attribute::Vector2f32 => gfx_hal::format::Format::Rg32Sfloat,
      Attribute::Vector3f32 => gfx_hal::format::Format::Rgb32Sfloat,
      Attribute::Vector4f32 => gfx_hal::format::Format::Rgba32Sfloat,
    }
  }
}

// Provide default implementations of `Data`.

impl Data for Color {
  const ATTRIBUTES: &'static [Attribute] = &[Attribute::Vector4f32];
}

impl Data for Point2<f32> {
  const ATTRIBUTES: &'static [Attribute] = &[Attribute::Vector2f32];
}
