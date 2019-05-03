// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use glsl_to_spirv::ShaderType as ShaderKind;

use crate::gpu::{Gpu, GpuDeviceExt as _};
use crate::Backend;
use std::ops::Deref;

pub type HalEntryPoint<'a> = gfx_hal::pso::EntryPoint<'a, Backend>;

type HalShader = <Backend as gfx_hal::Backend>::ShaderModule;

/// A compiled shader module on the device.
pub struct Shader {
  shader: HalShader,
}

impl Shader {
  pub fn new(gpu: &Gpu, code: &ShaderCode) -> Shader {
    let shader = unsafe {
      gpu
        .device
        .create_shader_module(code)
        .expect("Could not create shader module")
    };

    Shader { shader }
  }

  pub(crate) fn hal_entrypoint(&self) -> HalEntryPoint {
    HalEntryPoint {
      module: &self.shader,
      entry: "main",
      specialization: Default::default(),
    }
  }

  pub fn destroy(self, gpu: &Gpu) {
    unsafe { gpu.device.destroy_shader_module(self.shader) };
  }
}

pub struct ShaderCode(Vec<u8>);

impl ShaderCode {
  pub fn compile(kind: ShaderKind, source: impl AsRef<str>) -> Result<Self, String> {
    use std::io::Read;

    let mut output = glsl_to_spirv::compile(source.as_ref(), kind.clone())?;

    let mut spirv = Vec::with_capacity(output.metadata().map(|m| m.len()).unwrap_or(8192) as usize);

    output
      .read_to_end(&mut spirv)
      .expect("Could not read compiled shader");

    Ok(ShaderCode(spirv))
  }
}

impl Deref for ShaderCode {
  type Target = [u8];

  fn deref(&self) -> &[u8] {
    &self.0
  }
}
