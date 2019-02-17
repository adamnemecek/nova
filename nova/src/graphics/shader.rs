// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::device::{Device, RawDeviceExt};
use crate::graphics::Backend;
use crate::utils::Droppable;
use std::sync::Arc;

pub use glsl_to_spirv::ShaderType as ShaderKind;

type RawShader = <Backend as gfx_hal::Backend>::ShaderModule;

/// A compiled shader module on the device.
#[derive(Clone)]
pub struct Shader {
  inner: Arc<Inner>,
}

struct Inner {
  raw: Droppable<RawShader>,
  kind: ShaderKind,
  device: Device,
}

impl Shader {
  /// Creates a new shader on the device from the given compiled SPIR-V.
  pub fn new(device: &Device, spirv: &Spirv) -> Shader {
    let module = unsafe {
      device
        .raw()
        .create_shader_module(&spirv.1)
        .expect("Could not create backend shader module")
    };

    Shader {
      inner: Arc::new(Inner {
        device: device.clone(),
        raw: module.into(),
        kind: spirv.0.clone(),
      }),
    }
  }

  pub fn kind(&self) -> &ShaderKind {
    &self.inner.kind
  }

  pub(crate) fn raw(&self) -> &RawShader {
    &self.inner.raw
  }
}

impl Drop for Inner {
  fn drop(&mut self) {
    if let Some(raw) = self.raw.take() {
      unsafe {
        self.device.raw().destroy_shader_module(raw);
      }
    }
  }
}

pub struct ShaderSet {
  pub vertex: Shader,
  pub fragment: Option<Shader>,
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
