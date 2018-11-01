use super::backend;
use super::Context;
use gfx_hal::Device;
use std::sync::Arc;

pub struct RenderPass {
  raw: Option<backend::RenderPass>,
  context: Arc<Context>,
}

impl RenderPass {
  pub fn new(context: &Arc<Context>, format: gfx_hal::format::Format) -> Arc<Self> {
    let color_attachment = gfx_hal::pass::Attachment {
      format: Some(format),
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

    let raw = context
      .device
      .create_render_pass(&[color_attachment], &[subpass], &[dependency]);

    Arc::new(RenderPass {
      context: context.clone(),
      raw: Some(raw),
    })
  }

  pub fn context(&self) -> &Arc<Context> {
    &self.context
  }

  pub fn raw(&self) -> &backend::RenderPass {
    self.raw.as_ref().expect("render pass was destroyed")
  }
}

impl Drop for RenderPass {
  fn drop(&mut self) {
    if let Some(pass) = self.raw.take() {
      self.context.device.destroy_render_pass(pass);
    }
  }
}
