// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod image;
mod read;
mod slice;
mod write;

pub use self::image::Image;
pub use self::read::ReadImages;
pub use self::slice::ImageSlice;
pub use self::write::WriteImages;
pub use ::image::{ImageError, ImageFormat};

use nova_core::ecs;
use nova_core::engine::Engine;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageId(ecs::Entity);

impl From<ImageId> for ecs::Entity {
  fn from(id: ImageId) -> Self {
    id.0
  }
}

pub fn setup(engine: &mut Engine) {
  ecs::components::register::<Image>(&mut engine.resources);
}

pub fn read(res: &ecs::Resources) -> ReadImages {
  ecs::SystemData::fetch(res)
}

pub fn write(res: &ecs::Resources) -> WriteImages {
  ecs::SystemData::fetch(res)
}
