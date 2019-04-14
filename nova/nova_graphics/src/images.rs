// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod changes;
mod data;
mod image;
mod slice;

pub use self::data::ImageData;
pub(crate) use self::image::Image;
pub use self::slice::ImageSlice;
pub use gfx_hal::format::Format as ImageFormat;
pub use gfx_hal::image::Access as ImageAccess;
pub use gfx_hal::image::Layout as ImageLayout;

use self::changes::ImageChange;
use crate::commands::CommandBuffer;
use crate::gpu::Gpu;
use crate::pipelines::PipelineStage;
use crate::Color4;
use nova_core::collections::stash::{self, UniqueStash};
use nova_core::resources::{self, ReadResource, Resources, WriteResource};
use std::mem;
use std::ops::Range;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ImageId(stash::Tag);

pub type ReadImages<'a> = ReadResource<'a, Images>;
pub type WriteImages<'a> = WriteResource<'a, Images>;

#[derive(Debug, Default)]
pub struct Images {
  images: UniqueStash<Image>,
  changes: Vec<ImageChange>,
}

impl Images {
  pub fn get(&self, id: ImageId) -> Option<&Image> {
    self.images.get(id.0)
  }

  pub fn transition_image(
    &mut self,
    id: ImageId,
    stage: PipelineStage,
    access: Range<ImageAccess>,
    layout: Range<ImageLayout>,
  ) {
    self
      .changes
      .push(ImageChange::new_transition(id, stage, access, layout))
  }

  pub fn clear_image(
    &mut self,
    id: ImageId,
    color: Color4,
    stage: PipelineStage,
    access: Range<ImageAccess>,
    layout: Range<ImageLayout>,
  ) {
    self
      .changes
      .push(ImageChange::new_clear(id, color, stage, access, layout))
  }

  pub(crate) fn insert(&mut self, image: Image) -> ImageId {
    ImageId(self.images.put(image))
  }

  pub(crate) fn flush_changes(&mut self, cmd: &mut CommandBuffer) {
    for change in self.changes.drain(..) {
      if let Some(image) = self.images.get(change.image_id.0) {
        change.record(image, cmd);
      }
    }
  }

  pub(crate) fn destroy_image(&mut self, gpu: &Gpu, id: ImageId) {
    if let Some(image) = self.images.take(id.0) {
      image.destroy(gpu);
    }
  }

  pub(crate) fn destroy_all(&mut self, gpu: &Gpu) {
    let mut images = Default::default();

    mem::swap(&mut self.images, &mut images);

    for (_, image) in images {
      image.destroy(gpu);
    }
  }
}

pub fn borrow(res: &Resources) -> ReadImages {
  resources::borrow(res)
}

pub fn borrow_mut(res: &Resources) -> WriteImages {
  resources::borrow_mut(res)
}

pub(crate) fn set_up(res: &mut Resources) {
  res.entry().or_insert_with(Images::default);
}
