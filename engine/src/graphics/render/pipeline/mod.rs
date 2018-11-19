// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod builder;

pub use self::builder::{BuildError, PipelineBuilder};
pub use gfx_hal::pso::PipelineStage;

use super::descriptor::DescriptorLayout;
use super::shader::ShaderSet;
use crate::graphics::prelude::*;
use crate::graphics::Device;
use crate::utils::Droppable;
use std::ops::Range;
use std::sync::Arc;

/// A graphics pipeline that configures rendering with input descriptors, push
/// constants, and shaders.
pub struct Pipeline {
  device: Arc<Device>,
  raw: Droppable<(backend::GraphicsPipeline, backend::PipelineLayout)>,
  push_constants: Vec<Range<u32>>,
  descriptor_layouts: Vec<Arc<DescriptorLayout>>,
  shaders: ShaderSet,
}

impl Pipeline {
  /// Gets a reference to the raw backend layout of the pipeline.
  pub fn raw_layout(&self) -> &backend::PipelineLayout {
    &self.raw.1
  }

  /// Gets the range of a push constant by index.
  pub fn push_constant_range(&self, index: usize) -> Range<u32> {
    self.push_constants[index].clone()
  }

  /// Gets a list of references to the descriptor layouts defined in the
  /// pipeline.
  pub fn descriptor_layouts(&self) -> &[Arc<DescriptorLayout>] {
    &self.descriptor_layouts
  }

  /// Gets a reference to the shader set used in the pipeline.
  pub fn shaders(&self) -> &ShaderSet {
    &self.shaders
  }
}

// Implement `AsRef` to expose the raw backend graphics pipeline.
impl AsRef<backend::GraphicsPipeline> for Pipeline {
  fn as_ref(&self) -> &backend::GraphicsPipeline {
    &self.raw.0
  }
}

/// Implement `Drop` to destroy the raw backend resources.
impl Drop for Pipeline {
  fn drop(&mut self) {
    let device = self.device.raw();

    if let Some((pipeline, layout)) = self.raw.take() {
      device.destroy_pipeline_layout(layout);
      device.destroy_graphics_pipeline(pipeline);
    }
  }
}
