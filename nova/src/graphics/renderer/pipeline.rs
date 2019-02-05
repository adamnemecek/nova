// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod builder;
mod vertex;

use super::ShaderSet;
use crate::graphics::device::{Device, RawDeviceExt};
use crate::graphics::Backend;
use crate::utils::Droppable;
use std::ops::Range;
use std::sync::Arc;

pub use self::builder::{BuildError, PipelineBuilder};
pub use self::vertex::*;
pub use gfx_hal::pso::PipelineStage;

type RawPipeline = <Backend as gfx_hal::Backend>::GraphicsPipeline;
type RawPipelineLayout = <Backend as gfx_hal::Backend>::PipelineLayout;

/// A graphics pipeline that configures rendering with input descriptors, push
/// constants, and shaders.
#[derive(Clone)]
pub struct Pipeline {
  inner: Arc<Inner>,
}

struct Inner {
  device: Device,
  raw: Droppable<RawPipeline>,
  raw_layout: Droppable<RawPipelineLayout>,
  push_constants: Vec<Range<u32>>,
  _shaders: ShaderSet,
}

impl Pipeline {
  pub(crate) fn push_constant_range(&self, index: usize) -> Range<u32> {
    self.inner.push_constants[index].clone()
  }

  pub(crate) fn raw(&self) -> &RawPipeline {
    &self.inner.raw
  }

  pub(crate) fn raw_layout(&self) -> &RawPipelineLayout {
    &self.inner.raw_layout
  }
}

/// Implement `Drop` to destroy the raw backend resources.
impl Drop for Inner {
  fn drop(&mut self) {
    let device = self.device.raw();

    if let Some(raw) = self.raw.take() {
      unsafe {
        device.destroy_graphics_pipeline(raw);
      }
    }

    if let Some(raw_layout) = self.raw_layout.take() {
      unsafe {
        device.destroy_pipeline_layout(raw_layout);
      }
    }
  }
}
