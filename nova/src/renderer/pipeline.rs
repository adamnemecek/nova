// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod builder;
mod vertex;

use super::device::{Device, DeviceExt};
use super::{Backend, ShaderSet};
use std::ops::Range;

pub use self::builder::PipelineBuilder;
pub use self::vertex::*;
pub use gfx_hal::pso::PipelineStage;

type RawPipeline = <Backend as gfx_hal::Backend>::GraphicsPipeline;
type RawPipelineLayout = <Backend as gfx_hal::Backend>::PipelineLayout;

/// A graphics pipeline that configures rendering with input descriptors, push
/// constants, and shaders.
pub struct Pipeline {
  push_constants: Vec<Range<u32>>,
  pub(crate) raw_layout: RawPipelineLayout,
  pub(crate) raw: RawPipeline,
  shaders: ShaderSet,
}

impl Pipeline {
  pub(crate) fn push_constant_range(&self, index: usize) -> Range<u32> {
    self.push_constants[index].clone()
  }

  pub fn destroy(self, device: &Device) {
    unsafe {
      device.destroy_graphics_pipeline(self.raw);
      device.destroy_pipeline_layout(self.raw_layout);
    }

    self.shaders.destroy(&device);
  }
}
