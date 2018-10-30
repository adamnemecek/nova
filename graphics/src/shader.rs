use super::backend;
use super::Context;
use gfx_hal::Device;
use std::sync::Arc;

pub struct Shader {
  context: Arc<Context>,
  module: Option<backend::ShaderModule>,
}

impl Shader {
  pub fn new(context: &Arc<Context>, bytes: &[u8]) -> Self {
    let module = context
      .device()
      .create_shader_module(bytes)
      .expect("could not create shader module");

    Shader {
      context: context.clone(),
      module: Some(module),
    }
  }

  pub fn module(&self) -> &backend::ShaderModule {
    self.module.as_ref().expect("shader module was destroyed")
  }
}

impl Drop for Shader {
  fn drop(&mut self) {
    if let Some(module) = self.module.take() {
      self.context.device().destroy_shader_module(module);
    }
  }
}
