// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::gpu::{Gpu, GpuDeviceExt as _};
use crate::shaders::ShaderCode;
use crate::Backend;

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

  pub(crate) fn as_hal(&self) -> &HalShader {
    &self.shader
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
