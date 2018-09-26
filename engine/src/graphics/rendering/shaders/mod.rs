use super::backend;
use gfx_hal::Device;

pub struct ShaderPair {
  pub vert: backend::ShaderModule,
  pub frag: backend::ShaderModule,
}

impl ShaderPair {
  pub fn destroy(self, device: &backend::Device) {
    device.destroy_shader_module(self.vert);
    device.destroy_shader_module(self.frag);
  }
}

pub fn create_default(device: &backend::Device) -> ShaderPair {
  let vert = include_bytes!("spirv/default.vert.spv");
  let frag = include_bytes!("spirv/default.frag.spv");

  ShaderPair {
    vert: device
      .create_shader_module(vert)
      .expect("could not create vert module"),
    frag: device
      .create_shader_module(frag)
      .expect("could not create frag module"),
  }
}
