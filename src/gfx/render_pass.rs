// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A description of a render pass, including the required attachments,
/// subpasses, and subpass dependencies.
pub struct RenderPass {
  context: Arc<Context>,
  render_pass: Expect<backend::RenderPass>,
}

impl RenderPass {
  pub const FORMAT: gfx_hal::format::Format = gfx_hal::format::Format::Bgra8Unorm;

  /// Creates a new default render pass.
  pub fn new(context: &Arc<Context>) -> Self {
    let color_attachment = gfx_hal::pass::Attachment {
      format: Some(Self::FORMAT),
      samples: 1,
      ops: gfx_hal::pass::AttachmentOps::new(
        gfx_hal::pass::AttachmentLoadOp::Clear,
        gfx_hal::pass::AttachmentStoreOp::Store,
      ),
      stencil_ops: gfx_hal::pass::AttachmentOps::DONT_CARE,
      layouts: gfx_hal::image::Layout::Undefined..gfx_hal::image::Layout::Present,
    };

    let subpass = gfx_hal::pass::SubpassDesc {
      colors: &[(0, gfx_hal::image::Layout::ColorAttachmentOptimal)],
      depth_stencil: None,
      inputs: &[],
      resolves: &[],
      preserves: &[],
    };

    let dependency = gfx_hal::pass::SubpassDependency {
      passes: gfx_hal::pass::SubpassRef::External..gfx_hal::pass::SubpassRef::Pass(0),
      stages: gfx_hal::pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT
        ..gfx_hal::pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
      accesses: gfx_hal::image::Access::empty()
        ..(gfx_hal::image::Access::COLOR_ATTACHMENT_READ
          | gfx_hal::image::Access::COLOR_ATTACHMENT_WRITE),
    };

    let render_pass = unsafe {
      context
        .device()
        .create_render_pass(&[color_attachment], &[subpass], &[dependency])
        .expect("failed to create render pass")
        .into()
    };

    Self { render_pass, context: context.clone() }
  }

  /// Returns a reference to the context the render pass was created in.
  pub fn context(&self) -> &Context {
    &self.context
  }

  /// Returns a reference to the underlying backend render pass.
  pub fn as_backend(&self) -> &backend::RenderPass {
    &self.render_pass
  }
}

impl Drop for RenderPass {
  fn drop(&mut self) {
    unsafe {
      self.context.device().destroy_render_pass(self.render_pass.take());
    }
  }
}
