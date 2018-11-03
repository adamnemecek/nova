use super::backend;
use super::prelude::*;
use super::Device;
pub use glsl_to_spirv::ShaderType as ShaderKind;
use std::sync::Arc;

pub struct Shader {
  raw: Option<backend::ShaderModule>,
  device: Arc<Device>,
}

impl Shader {
  pub fn from_glsl(device: &Arc<Device>, kind: ShaderKind, source: &str) -> Shader {
    use std::io::Read;

    let mut spirv = Vec::new();
    let mut output = glsl_to_spirv::compile(source, kind).expect("could not compile shader");

    output
      .read_to_end(&mut spirv)
      .expect("could not read compiled shader");

    let module = device
      .raw
      .create_shader_module(&spirv)
      .expect("could not create shader module");

    Shader {
      device: device.clone(),
      raw: Some(module),
    }
  }

  pub fn raw(&self) -> &backend::ShaderModule {
    self.raw.as_ref().expect("shader module was destroyed")
  }
}

impl Drop for Shader {
  fn drop(&mut self) {
    if let Some(module) = self.raw.take() {
      self.device.raw.destroy_shader_module(module);
    }
  }
}
