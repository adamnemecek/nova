// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Pass, Shader, ShaderKind, Spirv};
use crate::graphics::device::{Device, RawDeviceExt};
use crate::graphics::Backend;
use crate::utils::Droppable;

type RawPipeline = <Backend as gfx_hal::Backend>::GraphicsPipeline;
type RawPipelineLayout = <Backend as gfx_hal::Backend>::PipelineLayout;

pub struct Pipeline {
  raw: Droppable<RawPipeline>,
  raw_layout: Droppable<RawPipelineLayout>,
  _shaders: [Shader; 2],
  device: Device,
}

impl Pipeline {
  pub fn new(pass: &Pass) -> Self {
    let device = pass.device();

    let vertex_shader = Shader::new(
      &device,
      &Spirv::from_glsl(ShaderKind::Vertex, include_str!("shaders/default.vert")),
    );

    let fragment_shader = Shader::new(
      &device,
      &Spirv::from_glsl(ShaderKind::Fragment, include_str!("shaders/default.frag")),
    );

    let raw_layout = unsafe {
      device
        .raw()
        .create_pipeline_layout(&[], &[])
        .expect("Could not create pipeline layout")
    };

    let mut pipeline_desc = gfx_hal::pso::GraphicsPipelineDesc::new(
      gfx_hal::pso::GraphicsShaderSet {
        vertex: gfx_hal::pso::EntryPoint {
          module: vertex_shader.raw(),
          entry: "main",
          specialization: Default::default(),
        },
        fragment: Some(gfx_hal::pso::EntryPoint {
          module: fragment_shader.raw(),
          entry: "main",
          specialization: Default::default(),
        }),
        domain: None,
        geometry: None,
        hull: None,
      },
      gfx_hal::Primitive::TriangleList,
      gfx_hal::pso::Rasterizer::FILL,
      &raw_layout,
      gfx_hal::pass::Subpass {
        index: 0,
        main_pass: pass.raw(),
      },
    );

    pipeline_desc
      .blender
      .targets
      .push(gfx_hal::pso::ColorBlendDesc(
        gfx_hal::pso::ColorMask::ALL,
        gfx_hal::pso::BlendState::ALPHA,
      ));

    let raw = unsafe {
      device
        .raw()
        .create_graphics_pipeline(&pipeline_desc, None)
        .expect("Could not create graphics pipeline")
    };

    Pipeline {
      raw: raw.into(),
      raw_layout: raw_layout.into(),
      _shaders: [vertex_shader, fragment_shader],
      device: device.clone(),
    }
  }

  pub(crate) fn raw(&self) -> &RawPipeline {
    &self.raw
  }
}

impl Drop for Pipeline {
  fn drop(&mut self) {
    let device = self.device.raw();

    if let Some(raw) = self.raw.take() {
      unsafe {
        device.destroy_graphics_pipeline(raw);
      }
    }

    if let Some(raw_layout) = self.raw_layout.take() {
      unsafe {
        device.destroy_pipeline_layout(raw_layout);
      }
    }
  }
}
