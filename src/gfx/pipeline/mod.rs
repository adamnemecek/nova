// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod builder;
mod graphics;

pub use self::{builder::*, graphics::*};
pub use gfx_hal::pso::{CreationError, PipelineStage as Stage};

use super::*;

/// Container for all of the possible shaders in a pipeline.
#[derive(Default)]
struct ShaderSet {
  pub vertex: Option<shader::Module>,
  pub fragment: Option<shader::Module>,
}
