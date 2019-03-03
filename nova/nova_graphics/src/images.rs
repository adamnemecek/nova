// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod image;
mod read;
mod slice;

pub use self::image::Image;
pub use self::read::ReadImages;
pub use self::slice::ImageSlice;
pub use ::image::ImageError;

use nova_assets::AssetId;
use nova_core::ecs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageId(ecs::Entity);

impl From<ImageId> for ecs::Entity {
  fn from(id: ImageId) -> Self {
    id.0
  }
}

impl From<AssetId> for ImageId {
  fn from(id: AssetId) -> ImageId {
    ImageId(id.into())
  }
}
