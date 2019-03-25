// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod data;
mod image;
mod slice;

pub use self::data::ImageData;
pub(crate) use self::image::Image;
pub use self::slice::ImageSlice;
pub use gfx_hal::format::Format as ImageFormat;

use crate::gpu::{self, Gpu};
use nova_core::collections::stash::{self, UniqueStash};
use nova_core::resources::{self, ReadResource, Resources, WriteResource};
use std::mem;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ImageId(stash::Tag);

pub type ReadImages<'a> = ReadResource<'a, Images>;
pub type WriteImages<'a> = WriteResource<'a, Images>;

#[derive(Debug, Default)]
pub struct Images {
  images: UniqueStash<Image>,
}

impl Images {
  pub(crate) fn insert(&mut self, image: Image) -> ImageId {
    ImageId(self.images.put(image))
  }

  pub(crate) fn destroy_view(&mut self, gpu: &Gpu, id: ImageId) {
    if let Some(image) = self.images.take(id.0) {
      image.destroy_view(gpu);
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
