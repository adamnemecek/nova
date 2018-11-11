// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::backend;
use crate::graphics::hal::prelude::*;
use crate::graphics::image;
use crate::graphics::Device;
use std::sync::Arc;

pub struct RenderPass {
  device: Arc<Device>,
  raw: Option<backend::RenderPass>,
  format: image::Format,
}

impl RenderPass {
  pub fn new(device: &Arc<Device>) -> Arc<Self> {
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
