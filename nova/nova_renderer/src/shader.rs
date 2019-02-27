// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::device::{Device, DeviceExt};
use super::Backend;
use std::sync::Arc;

pub use glsl_to_spirv::ShaderType as ShaderKind;

type RawShader = <Backend as gfx_hal::Backend>::ShaderModule;

/// A compiled shader module on the device.
#[derive(Clone)]
pub struct Shader {
  inner: Arc<Inner>,
}

struct Inner {
  raw: RawShader,
  kind: ShaderKind,
}

impl Shader {
  /// Creates a new shader on the device from the given compiled SPIR-V.
  pub fn new(device: &Device, spirv: &Spirv) -> Shader {
    let raw = unsafe {
      device
        .create_shader_module(&spirv.1)
        .expect("Could not create backend shader module")
    };

    Shader {
      inner: Arc::new(Inner {
        raw,
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

  pub fn destroy(self, device: &Device) {
    if let Ok(inner) = Arc::try_unwrap(self.inner) {
      unsafe {
        device.destroy_shader_module(inner.raw);
      }
    }
  }
}

pub struct ShaderSet {
  pub vertex: Shader,
  pub fragment: Option<Shader>,
}

impl ShaderSet {
  pub fn destroy(self, device: &Device) {
    self.vertex.destroy(device);

    if let Some(fragment) = self.fragment {
      fragment.destroy(device);
    }
  }
}

/// A shader compiled to SPIR-V.
pub struct Spirv(ShaderKind, Vec<u8>);

impl Spirv {
  /// Creates a new compiled SPIR-V from a string containing GLSL source.
  pub fn from_glsl(kind: ShaderKind, source: impl AsRef<str>) -> Self {
    use std::io::Read;

    let mut output = glsl_to_spirv::compile(source.as_ref(), kind.clone())
      .expect("Could not compile shader");

    let mut spirv =
      Vec::with_capacity(output.metadata().map(|m| m.len()).unwrap_or(8192) as usize);

    output
      .read_to_end(&mut spirv)
      .expect("Could not read compiled shader");

    Spirv(kind, spirv)
  }
}
