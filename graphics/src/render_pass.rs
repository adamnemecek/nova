use super::backend;
use super::Context;
use gfx_hal::Device;
use std::sync::Arc;

pub struct RenderPass {
  pub context: Arc<Context>,
  pub pass: Option<backend::RenderPass>,
  pub log: bflog::Logger,
}

impl RenderPass {
  pub fn new(context: &Arc<Context>) -> Arc<Self> {
    let mut log = context.log().with_src("graphics::RenderPass");

    let color_attachment = gfx_hal::pass::Attachment {
      format: Some(context.format()),
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

    let pass = context
      .device()
      .create_render_pass(&[color_attachment], &[subpass], &[dependency]);

    log.trace("Created.");

    Arc::new(RenderPass {
      context: context.clone(),
      pass: Some(pass),
      log,
    })
  }

  pub fn context(&self) -> &Arc<Context> {
    &self.context
  }

  pub fn pass(&self) -> &backend::RenderPass {
    self.pass.as_ref().expect("render pass was destroyed")
  }
}

impl Drop for RenderPass {
  fn drop(&mut self) {
    if let Some(pass) = self.pass.take() {
      self.context.device().destroy_render_pass(pass);
      self.log.trace("Destroyed.");
    }
  }
}
