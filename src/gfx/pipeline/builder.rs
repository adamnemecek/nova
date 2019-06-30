// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A declarative builder for creating a `Pipeline`.
#[derive(Default)]
pub struct PipelineBuilder {
  pub(super) shaders: ShaderSet,
  pub(super) size_of_push_constants: usize,
  pub(super) render_pass: Option<RenderPass>,
  pub(super) vertex_buffers: Vec<gfx_hal::pso::VertexBufferDesc>,
  pub(super) vertex_attributes: Vec<gfx_hal::pso::AttributeDesc>,
  pub(super) desriptor_layouts: Vec<DescriptorLayout>,
}

impl PipelineBuilder {
  /// Creates a new builder.
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the render pass of the pipeline.
  pub fn set_render_pass(mut self, render_pass: RenderPass) -> Self {
    self.render_pass = render_pass.into();
    self
  }

  /// Sets the vertex shader of the pipeline.
  pub fn set_vertex_shader(mut self, module: shader::Module) -> Self {
    self.shaders.vertex = module.into();
    self
  }

  /// Sets the fragment shader of the pipeline.
  pub fn set_fragment_shader(mut self, module: shader::Module) -> Self {
    self.shaders.fragment = module.into();
    self
  }

  /// Sets the push constants type of the pipeline to `T`.
  pub fn set_push_constants<T: Sized>(mut self) -> Self {
    self.size_of_push_constants = mem::size_of::<T>();
    self
  }

  /// Adds a vertex buffer of type `T` to the pipeline.
  pub fn add_vertex_buffer<T: vertex::Data>(mut self) -> Self {
    let binding = self.vertex_buffers.len() as u32;

    self.vertex_buffers.push(gfx_hal::pso::VertexBufferDesc {
      binding,
      stride: T::stride(),
      rate: gfx_hal::pso::VertexInputRate::Vertex,
    });

    let mut offset = 0;

    for attribute in T::ATTRIBUTES {
      self.vertex_attributes.push(gfx_hal::pso::AttributeDesc {
        binding,
        location: self.vertex_attributes.len() as u32,
        element: gfx_hal::pso::Element { format: attribute.backend_format(), offset },
      });

      offset += attribute.size();
    }

    self
  }

  /// Adds a [`DescriptorSet`] to the pipeline with the given `layout`.
  pub fn add_descriptor_set(mut self, layout: DescriptorLayout) -> Self {
    self.desriptor_layouts.push(layout);
    self
  }

  /// Builds a graphics pipeline as configured.
  pub fn into_graphics(self, context: &Arc<Context>) -> Result<Graphics, GraphicsError> {
    Graphics::new(context, self)
  }
}
