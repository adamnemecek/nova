// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

use image;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Provides access to files in the `assets` directory.
pub struct Assets {
  pub path: PathBuf,
}

impl Assets {
  pub fn load<T: Asset>(&self, path: &Path) -> Result<T, Box<dyn Error>> {
    Asset::load(self.path.join(path))
  }

  pub fn save<T: Asset>(&self, path: &Path, asset: &T) -> Result<(), Box<dyn Error>> {
    asset.save(self.path.join(path))
  }
}

impl Default for Assets {
  fn default() -> Self {
    let mut path = env::var("CARGO_MANIFEST_DIR")
      .map(PathBuf::from)
      .unwrap_or_else(|_| {
        let mut path = env::current_exe().expect("could not get current exe path");

        path.pop();
        path
      });

    path.push("assets");

    Assets { path }
  }
}

pub trait Asset
where
  Self: Sized,
{
  fn load(path: PathBuf) -> Result<Self, Box<dyn Error>>;
  fn save(&self, path: PathBuf) -> Result<(), Box<dyn Error>>;
}

impl<T> Asset for T
where
  T: Serialize,
  for<'de> T: Deserialize<'de>,
{
  fn load(path: PathBuf) -> Result<Self, Box<dyn Error>> {
    let file = File::open(path)?;

    Ok(serde_yaml::from_reader(file)?)
  }

  fn save(&self, path: PathBuf) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;

    Ok(serde_yaml::to_writer(file, self)?)
  }
}

/// Load a ggez `Image` from the given `path` relative to the core's asset
/// directory.
pub fn load_image(core: &mut Core, path: &Path) -> Result<ggez::graphics::Image, Box<dyn Error>> {
  let img = {
    let mut buf = Vec::new();
    let mut file = File::open(core.world.read_resource::<Assets>().path.join(path))?;

    file.read_to_end(&mut buf)?;
    image::load_from_memory(&buf)?.to_rgba()
  };

  let (width, height) = img.dimensions();
  let mut image =
    ggez::graphics::Image::from_rgba8(&mut core.ctx, width as u16, height as u16, &img)?;

  image.set_filter(ggez::graphics::FilterMode::Nearest);

  Ok(image)
}
