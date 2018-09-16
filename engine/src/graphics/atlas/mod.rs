// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ggez::graphics::{FilterMode, Image, Rect};
use serde_yaml;
use std::error::Error;
use std::path::PathBuf;

use prelude::*;

pub mod animation;
pub mod data;

pub use self::animation::Animation;
pub use self::data::Data;

/// An image split into one or more cells.
///
/// Also known as a spritesheet.
pub struct Atlas {
  pub image: Image,
  pub data: Data,
}

impl Atlas {
  pub fn load(core: &mut Core, path: impl Into<PathBuf>) -> Result<Self, Box<dyn Error>> {
    let mut path = path.into();

    // Append `.png` to the path and load it as the image.
    path.set_extension("png");

    let mut image = Image::new(&mut core.ctx, &path)?;

    image.set_filter(FilterMode::Nearest);

    // Append `.yml` to the path and attempt to load it as `Data`.
    path.set_extension("yml");

    if let Ok(file) = ggez::filesystem::open(&mut core.ctx, &path) {
      let data = serde_yaml::from_reader::<_, Data>(file)?;

      Ok(Self { image, data })
    } else {
      let data = Data::new(image.width() as usize, image.height() as usize);

      Ok(Self { image, data })
    }
  }

  pub fn get(&self, cell: usize) -> Rect {
    let width = self.image.width() as f32;

    let w = self.data.cell_width as f32 / width;
    let h = self.data.cell_height as f32 / self.image.height() as f32;

    let mut x = cell as f32 * w;
    let y = x.floor() * h;

    x %= 1.0;

    Rect::new(x, y, w, h)
  }

  pub fn get_animation(&self, name: &str) -> Option<usize> {
    for (i, animation) in self.data.animations.iter().enumerate() {
      if animation.name == name {
        return Some(i);
      }
    }

    None
  }
}
