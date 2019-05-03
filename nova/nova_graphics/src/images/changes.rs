// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::gpu::CommandBuffer;
use crate::images::{Image, ImageAccess, ImageId, ImageLayout};
use crate::rendering::{MemoryBarrier, PipelineStage};
use crate::{Backend, Color4};
use std::ops::Range;

#[derive(Debug)]
pub struct ImageChange {
  pub(crate) image_id: ImageId,
  stage: PipelineStage,
  access: Range<ImageAccess>,
  layout: Range<ImageLayout>,
  op: Op,
}

impl ImageChange {
  pub fn new_transition(
    image_id: ImageId,
    stage: PipelineStage,
    access: Range<ImageAccess>,
    layout: Range<ImageLayout>,
  ) -> Self {
    Self {
      image_id,
      stage,
      access,
      layout,
      op: Op::Transition,
    }
  }

  pub fn new_clear(
    image_id: ImageId,
    color: Color4,
    stage: PipelineStage,
    access: Range<ImageAccess>,
    layout: Range<ImageLayout>,
  ) -> Self {
    Self {
      image_id,
      stage,
      access,
      layout,
      op: Op::Clear(color),
    }
  }

  pub fn record(self, image: &Image, cmd: &mut CommandBuffer) {
    match self.op {
      Op::Transition => {
        cmd.pipeline_barrier(
          self.stage..self.stage,
          Some(memory_barrier(image, self.access, self.layout)),
        );
      }

      Op::Clear(color) => {
        cmd.pipeline_barrier(
          self.stage..PipelineStage::TRANSFER,
          Some(memory_barrier(
            image,
            self.access.start..ImageAccess::TRANSFER_WRITE,
            self.layout.start..ImageLayout::TransferDstOptimal,
          )),
        );

        cmd.clear_image(image, color);

        cmd.pipeline_barrier(
          PipelineStage::TRANSFER..self.stage,
          Some(memory_barrier(
            image,
            ImageAccess::TRANSFER_WRITE..self.access.end,
            ImageLayout::TransferDstOptimal..self.layout.end,
          )),
        );
      }
    }
  }
}

#[derive(Debug)]
enum Op {
  Transition,
  Clear(Color4),
}

fn memory_barrier(
  target: &Image,
  access_change: Range<ImageAccess>,
  layout_change: Range<ImageLayout>,
) -> MemoryBarrier<Backend> {
  MemoryBarrier::Image {
    families: None,
    target: target.as_hal(),
    states: (access_change.start, layout_change.start)..(access_change.end, layout_change.end),
    range: gfx_hal::image::SubresourceRange {
      aspects: gfx_hal::format::Aspects::COLOR,
      levels: 0..1,
      layers: 0..1,
    },
  }
}
