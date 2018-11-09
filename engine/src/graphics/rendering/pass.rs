use super::*;
use crate::graphics::image;
use std::sync::Arc;

pub struct RenderPass {
  device: Arc<Device>,
  raw: Option<backend::RenderPass>,
  format: image::Format,
}

impl RenderPass {
  pub fn new(device: &Arc<Device>) -> Arc<Self> {
    let format = image::Format::Bgra8Unorm;

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

    let pass = device
      .raw()
      .create_render_pass(&[color_attachment], &[subpass], &[dependency])
      .expect("could not create render pass");

    Arc::new(RenderPass {
      device: device.clone(),
      raw: Some(pass),
      format,
    })
  }

  pub fn device(&self) -> &Arc<Device> {
    &self.device
  }

  pub fn raw(&self) -> &backend::RenderPass {
    self.raw.as_ref().expect("render pass is destroyed")
  }

  pub fn format(&self) -> image::Format {
    self.format
  }
}

impl Drop for RenderPass {
  fn drop(&mut self) {
    if let Some(pass) = self.raw.take() {
      self.device.raw().destroy_render_pass(pass);
    }
  }
}
