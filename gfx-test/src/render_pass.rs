use super::Context;
use gfx_hal::format::Format;
use gfx_hal::image::{Access, Layout};
use gfx_hal::pass::{Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp};
use gfx_hal::pass::{SubpassDependency, SubpassDesc, SubpassRef};
use gfx_hal::pso::PipelineStage;
use gfx_hal::Device;

pub use super::gfx_back::RenderPass;

pub fn create(ctx: &mut Context, surface_format: Format) -> RenderPass {
  // Struct describing an image attached to the render pass.
  let color_attachment = Attachment {
    // Use the given surface format.
    format: Some(surface_format),
    // Don't need more than one sample yet.
    //
    // TODO: Do I ever?
    samples: 1,
    // Clear the image before rendering, store it after so it can be
    // displayed.
    ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
    // Don't use a stencil.
    //
    // TODO: Can I use depth testing for 2D? How does that affect UI?
    stencil_ops: AttachmentOps::DONT_CARE,
    // Don't care what the old layout of an image is (since it gets cleared),
    // but the final layout should be present-ready.
    layouts: Layout::Undefined..Layout::Present,
  };

  // Struct describing the single subpass of the render pass.
  let subpass = SubpassDesc {
    // Use the above image attachment with the optimal layout.
    colors: &[(0, Layout::ColorAttachmentOptimal)],
    depth_stencil: None,
    inputs: &[],
    resolves: &[],
    preserves: &[],
  };

  // Struct describing the dependencies of the subpass.
  let dependency = SubpassDependency {
    passes: SubpassRef::External..SubpassRef::Pass(0),
    stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT..PipelineStage::COLOR_ATTACHMENT_OUTPUT,
    accesses: Access::empty()..(Access::COLOR_ATTACHMENT_READ | Access::COLOR_ATTACHMENT_WRITE),
  };

  ctx
    .device
    .create_render_pass(&[color_attachment], &[subpass], &[dependency])
}
