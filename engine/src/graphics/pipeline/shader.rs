// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use glsl_to_spirv::ShaderType as ShaderKind;

use crate::graphics::backend;
use crate::graphics::hal::prelude::*;
use crate::graphics::Device;
use crate::utils::Droppable;
use derive_more::*;
use std::sync::Arc;

/// A compiled shader module on the graphics device.
pub struct Shader {
  device: Arc<Device>,
  raw: Droppable<backend::ShaderModule>,
  kind: ShaderKind,
}

impl Shader {
  /// Creates a new shader on the device from the given compiled SPIR-V.
  pub fn new(device: &Arc<Device>, spirv: &Spirv) -> Shader {
    let module = device
      .raw()
      .create_shader_module(&spirv.1)
      .expect("Could not create backend shader module");

    Shader {
      device: device.clone(),
      raw: module.into(),
      kind: spirv.0.clone(),
    }
  }

  /// Gets the kind of shader.
  pub fn kind(&self) -> &ShaderKind {
    &self.kind
  }
}

// Implement `AsRef` to expose the raw backend shader module.
impl AsRef<backend::ShaderModule> for Shader {
  fn as_ref(&self) -> &backend::ShaderModule {
    &self.raw
  }
}

// Implement `Drop` to destroy the raw backend shader module.
impl Drop for Shader {
  fn drop(&mut self) {
    if let Some(module) = self.raw.take() {
      self.device.raw().destroy_shader_module(module);
    }
  }
}

/// A reference to an entry point function in a particular shader.
#[derive(From)]
pub struct EntryPoint(pub Arc<Shader>, pub String);

// Implement `From` to convert `&EntryPoint` to the equivalent gfx-hal structure.
impl<'a> From<&'a EntryPoint> for hal::pso::EntryPoint<'a> {
  fn from(point: &'a EntryPoint) -> Self {
    hal::pso::EntryPoint {
      module: point.0.as_ref().as_ref(),
      entry: &point.1,
      specialization: Default::default(),
    }
  }
}

/// A set of shader entry points for use in a pipeline.
pub struct ShaderSet {
  pub vertex: EntryPoint,
  pub fragment: Option<EntryPoint>,
}

// Implement `From` to convert `&ShaderSet` to the equivalent gfx-hal structure.
impl<'a> From<&'a ShaderSet> for hal::pso::GraphicsShaderSet<'a> {
  fn from(set: &'a ShaderSet) -> Self {
    hal::pso::GraphicsShaderSet {
      vertex: (&set.vertex).into(),
      hull: None,
      domain: None,
      geometry: None,
      fragment: set.fragment.as_ref().map(Into::into),
    }
  }
}

/// A shader compiled to SPIR-V.
pub struct Spirv(ShaderKind, Vec<u8>);

impl Spirv {
  /// Creates a new compiled SPIR-V from a string containing GLSL source.
  pub fn from_glsl(kind: ShaderKind, source: impl AsRef<str>) -> Self {
    use std::io::Read;

    let mut output =
      glsl_to_spirv::compile(source.as_ref(), kind.clone()).expect("Could not compile shader");

    let mut spirv = Vec::with_capacity(output.metadata().map(|m| m.len()).unwrap_or(8192) as usize);

    output
      .read_to_end(&mut spirv)
      .expect("Could not read compiled shader");

    Spirv(kind, spirv)
  }
}
