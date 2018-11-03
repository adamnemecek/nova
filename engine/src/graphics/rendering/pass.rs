use super::backend;
use super::prelude::*;
use super::{Destroy, Destroyable, Device};
use std::sync::Arc;

pub struct RenderPass(Destroyable<Inner>);

struct Inner {
  raw: backend::RenderPass,
  device: Arc<Device>,
}

impl RenderPass {
  pub fn new(device: &Arc<Device>) -> Arc<Self> {
    let color_attachment = gfx_hal::pass::Attachment {
      format: Some(gfx_hal::format::Format::Bgra8Srgb),
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

    let pass = device
      .raw
      .create_render_pass(&[color_attachment], &[subpass], &[dependency]);

    Arc::new(RenderPass(
      Inner {
        raw: pass,
        device: device.clone(),
      }
      .into(),
    ))
  }

  pub fn raw(&self) -> &backend::RenderPass {
    &self.0.raw
  }
}

impl Destroy for Inner {
  fn destroy(self) {
    self.device.raw.destroy_render_pass(self.raw);
  }
}
