// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod layout;
mod set;

pub use self::layout::*;
pub use self::set::*;

use crate::graphics::image::{self, Image};
use crate::graphics::prelude::*;
use std::sync::Arc;

/// A reference to data that can be bound to a command buffer and used in
/// pipeline shaders.
pub enum Descriptor {
  /// A combined image and sampler exposed in the shader as a texture.
  Texture(Arc<Image>, Arc<image::Sampler>),
}

// Implement `From` for references to descriptors to convert them to the
// equivalent gfx-hal structure.
impl<'a> From<&'a Descriptor> for hal::pso::Descriptor<'a, Backend> {
  fn from(desc: &'a Descriptor) -> Self {
    match desc {
      Descriptor::Texture(ref image, ref sampler) => hal::pso::Descriptor::CombinedImageSampler(
        image.as_ref().as_ref(), // &Arc<Image> -> &Image -> &backend::Image
        hal::image::Layout::ShaderReadOnlyOptimal,
        sampler.raw(),
      ),
    }
  }
}
