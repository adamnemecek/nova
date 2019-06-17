// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::backend;
use crate::Context;
use gfx_hal::Device as _;
use std::cmp;
use std::sync::Arc;

/// Describes a single pass of the renderer.
///
/// This structure is cloneable and all clones refer to the same render pass.
/// When all clones are dropped, the underyling backend resources are destroyed.
#[derive(Clone)]
pub struct RenderPass(Arc<RenderPassInner>);

struct RenderPassInner {
  context: Context,
  pass: Option<backend::RenderPass>,
}

impl RenderPass {
  pub(crate) const FORMAT: gfx_hal::format::Format = gfx_hal::format::Format::Bgra8Unorm;

  /// Creates a new default render pass.
  pub fn new(context: &Context) -> Self {
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

    let pass = unsafe {
      context
        .device
        .create_render_pass(&[color_attachment], &[subpass], &[dependency])
        .expect("failed to create render pass")
        .into()
    };

    RenderPass(Arc::new(RenderPassInner {
      pass,
      context: context.clone(),
    }))
  }

  /// Returns a reference to the context the render pass was created in.
  pub(crate) fn context(&self) -> &Context {
    &self.0.context
  }

  /// Returns a reference to the underlying backend render pass.
  pub(crate) fn as_backend(&self) -> &backend::RenderPass {
    self.0.pass.as_ref().unwrap()
  }
}

impl Drop for RenderPassInner {
  fn drop(&mut self) {
    unsafe {
      self
        .context
        .device
        .destroy_render_pass(self.pass.take().unwrap());
    }
  }
}

impl cmp::PartialEq for RenderPass {
  fn eq(&self, other: &RenderPass) -> bool {
    Arc::ptr_eq(&self.0, &other.0)
  }
}
