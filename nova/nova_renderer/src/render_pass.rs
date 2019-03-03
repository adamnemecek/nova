// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::images::DeviceImageFormat;
use super::Backend;
use super::{Device, DeviceExt};

pub type RenderPass = <Backend as gfx_hal::Backend>::RenderPass;

pub fn create(device: &Device) -> RenderPass {
  const FORMAT: DeviceImageFormat = DeviceImageFormat::Bgra8Unorm;

  let color_attachment = gfx_hal::pass::Attachment {
    format: Some(FORMAT),
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

  unsafe {
    device
      .create_render_pass(&[color_attachment], &[subpass], &[dependency])
      .expect("Could not create render pass")
  }
}
