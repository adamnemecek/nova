use super::gfx_back;
use super::gfx_back::ShaderModule;
use gfx_hal::Device;

pub struct Shaders {
  pub vert: ShaderModule,
  pub frag: ShaderModule,
}

pub fn create(device: &gfx_back::Device) -> Shaders {
  let vert = include_bytes!("../../spirv/default.vert.spv");
  let frag = include_bytes!("../../spirv/default.frag.spv");

  Shaders {
    vert: device
      .create_shader_module(vert)
      .expect("could not create ver module"),
    frag: device
      .create_shader_module(frag)
      .expect("could not create frag module"),
  }
}

pub fn destroy(device: &gfx_back::Device, shaders: Shaders) {
  device.destroy_shader_module(shaders.vert);
  device.destroy_shader_module(shaders.frag);
}
