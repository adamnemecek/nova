// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
pub use gfx_hal::pso::Stage;
use std::io::Read as _;

#[derive(Clone)]
pub struct Module(Arc<ModuleInner>);

struct ModuleInner {
  context: Context,
  shader: Option<backend::Shader>,
}

impl Module {
  pub(crate) fn from_backend(context: &Context, shader: backend::Shader) -> Self {
    Self(Arc::new(ModuleInner {
      context: context.clone(),
      shader: Some(shader),
    }))
  }

  /// ( ͡° ͜ʖ ͡°)
  pub(crate) fn backend_entrypoint(&self) -> backend::EntryPoint {
    backend::EntryPoint {
      entry: "main",
      module: self.0.shader.as_ref().unwrap(),
      specialization: Default::default(),
    }
  }
}

impl Drop for ModuleInner {
  fn drop(&mut self) {
    unsafe {
      self
        .context
        .device
        .destroy_shader_module(self.shader.take().unwrap());
    }
  }
}

pub fn compile_spirv(context: &Context, byte_code: &[u8]) -> Result<Module, CreationError> {
  let shader = unsafe { context.device.create_shader_module(byte_code)? };

  Ok(Module::from_backend(context, shader))
}

pub fn compile_hlsl(context: &Context, stage: Stage, code: &str) -> Result<Module, CreationError> {
  let mut output = glsl_to_spirv::compile(
    code,
    match stage {
      Stage::Vertex => glsl_to_spirv::ShaderType::Vertex,
      Stage::Fragment => glsl_to_spirv::ShaderType::Fragment,

      _ => {
        panic!("cannot compile shaders for stage {:?}", stage);
      }
    },
  )
  .map_err(|err| CreationError::CompilationFailed(err.to_string()))?;

  let mut spirv = Vec::with_capacity(output.metadata().map(|m| m.len()).unwrap_or(8192) as usize);

  output
    .read_to_end(&mut spirv)
    .expect("Could not read compiled shader");

  compile_spirv(context, &spirv)
}

#[derive(Debug)]
pub enum CreationError {
  /// The shader failed to compile.
  CompilationFailed(String),
  /// Out of either host or device memory.
  OutOfMemory,
}

impl From<gfx_hal::device::ShaderError> for CreationError {
  fn from(err: gfx_hal::device::ShaderError) -> Self {
    match err {
      gfx_hal::device::ShaderError::CompilationFailed(reason) => {
        CreationError::CompilationFailed(reason)
      }
      gfx_hal::device::ShaderError::OutOfMemory(_) => CreationError::OutOfMemory,
      err => panic!("failed to create shader module: {}", err),
    }
  }
}

impl fmt::Display for CreationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      CreationError::OutOfMemory => write!(f, "out of memory"),
      CreationError::CompilationFailed(err) => write!(f, "shader compilation failed: {}", err),
    }
  }
}
