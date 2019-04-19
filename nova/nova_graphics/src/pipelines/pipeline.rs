// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::gpu::Gpu;
use crate::images::Image;
use crate::render::RenderPass;
use crate::shaders::Shader;
use crate::Backend;
use gfx_hal::Device as _;
use nova_core::collections::SmallVec;
use nova_core::math::Size;
use std::iter;

type HalPipeline = <Backend as gfx_hal::Backend>::GraphicsPipeline;
type HalPipelineLayout = <Backend as gfx_hal::Backend>::PipelineLayout;
type HalDescriptorSetLayout = <Backend as gfx_hal::Backend>::DescriptorSetLayout;

pub struct Pipeline {
  pipeline: HalPipeline,
  layout: HalPipelineLayout,
  options: PipelineOptions,
}

impl Pipeline {
  pub fn new(gpu: &Gpu, render_pass: &RenderPass, options: PipelineOptions) -> Self {
    debug_assert!(
      options.size_of_push_constants % 4 == 0,
      "size_of_push_constants must be a multiple of 4"
    );

    let layout = unsafe {
      gpu
        .device
        .create_pipeline_layout(
          iter::empty::<HalDescriptorSetLayout>(),
          if options.size_of_push_constants > 0 {
            Some((
              gfx_hal::pso::ShaderStageFlags::ALL,
              0..options.size_of_push_constants as u32 / 4,
            ))
          } else {
            None
          },
        )
        .expect("Could not create pipeline layout")
    };

    let desc = gfx_hal::pso::GraphicsPipelineDesc::new(
      gfx_hal::pso::GraphicsShaderSet {
        vertex: options.vertex_shader.hal_entrypoint(),
        fragment: Some(options.vertex_shader.hal_entrypoint()),
        geometry: None,
        domain: None,
        hull: None,
      },
      gfx_hal::Primitive::TriangleStrip,
      gfx_hal::pso::Rasterizer::FILL,
      &layout,
      gfx_hal::pass::Subpass {
        index: 0,
        main_pass: render_pass.as_hal(),
      },
    );

    let pipeline = unsafe {
      gpu
        .device
        .create_graphics_pipeline(&desc, None)
        .expect("Could not create pipeline")
    };

    Self {
      pipeline,
      layout,
      options,
    }
  }

  pub fn destroy(self, gpu: &Gpu) {
    unsafe {
      gpu.device.destroy_graphics_pipeline(self.pipeline);
      gpu.device.destroy_pipeline_layout(self.layout);
    }

    self.options.vertex_shader.destroy(gpu);
    self.options.fragment_shader.destroy(gpu);
  }
}

pub struct PipelineOptions {
  pub vertex_shader: Shader,
  pub fragment_shader: Shader,
  pub size_of_push_constants: usize,
}
