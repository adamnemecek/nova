// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::{Backend, Device, ImageFormat, RawDeviceExt};
use crate::utils::Droppable;
use std::sync::Arc;

type RawRenderPass = <Backend as gfx_hal::Backend>::RenderPass;

#[derive(Clone)]
pub struct Pass {
  inner: Arc<Inner>,
}

impl Pass {
  pub fn new(device: &Device) -> Self {
    let format = ImageFormat::Bgra8Unorm;

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

    let raw = unsafe {
      device
        .raw()
        .create_render_pass(&[color_attachment], &[subpass], &[dependency])
        .expect("Could not create render pass")
    };

    Pass {
      inner: Arc::new(Inner {
        device: device.clone(),
        raw: raw.into(),
      }),
    }
  }

  pub fn device(&self) -> &Device {
    &self.inner.device
  }

  pub const fn attachment_count(&self) -> usize {
    1
  }

  pub(crate) fn raw(&self) -> &RawRenderPass {
    &self.inner.raw
  }
}

struct Inner {
  raw: Droppable<RawRenderPass>,
  device: Device,
}

impl Drop for Inner {
  fn drop(&mut self) {
    if let Some(raw) = self.raw.take() {
      unsafe {
        self.device.raw().destroy_render_pass(raw);
      }
    }
  }
}
