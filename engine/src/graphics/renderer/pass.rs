// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::device;
use crate::graphics::image;
use crate::graphics::prelude::*;

pub fn create_render_pass(device: &device::Handle) -> backend::RenderPass {
  let format = image::Format::Bgra8Unorm;

  let color_attachment = hal::pass::Attachment {
    format: Some(format),
    samples: 1,
    ops: hal::pass::AttachmentOps::new(
      hal::pass::AttachmentLoadOp::Clear,
      hal::pass::AttachmentStoreOp::Store,
    ),
    stencil_ops: hal::pass::AttachmentOps::DONT_CARE,
    layouts: hal::image::Layout::Undefined..hal::image::Layout::Present,
  };

  let subpass = hal::pass::SubpassDesc {
    colors: &[(0, hal::image::Layout::ColorAttachmentOptimal)],
    depth_stencil: None,
    inputs: &[],
    resolves: &[],
    preserves: &[],
  };

  let dependency = hal::pass::SubpassDependency {
    passes: hal::pass::SubpassRef::External..hal::pass::SubpassRef::Pass(0),
    stages: hal::pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT
      ..hal::pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
    accesses: hal::image::Access::empty()
      ..(hal::image::Access::COLOR_ATTACHMENT_READ | hal::image::Access::COLOR_ATTACHMENT_WRITE),
  };

  device
    .raw()
    .create_render_pass(&[color_attachment], &[subpass], &[dependency])
    .expect("could not create render pass")
}
