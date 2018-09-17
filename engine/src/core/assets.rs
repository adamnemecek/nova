// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

use serde::{Deserialize, Serialize};
use serde_yaml;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

/// Provides access to files in the `assets` directory.
pub struct Assets {
  pub path: PathBuf,
  send_resource_load: crossbeam_channel::Sender<Texture>,
  recv_resource_load: crossbeam_channel::Receiver<Texture>,
}

impl Assets {
  pub fn new(path: impl Into<PathBuf>) -> Self {
    let (send_resource_load, recv_resource_load) = crossbeam_channel::unbounded();

    Assets {
      path: path.into(),
      send_resource_load,
      recv_resource_load,
    }
  }

  pub fn open_file(&self, path: &Path) -> io::Result<File> {
    File::open(self.path.join(path))
  }

  pub fn queue_resource_load(&self, texture: Texture) {
    self.send_resource_load.send(texture);
  }

  pub fn load<T: Asset>(&self, path: &Path) -> Result<T, Box<dyn Error>> {
    T::load(self, path)
  }

  pub fn load_queued_resources(&self, ctx: &mut ggez::Context) {
    while let Some(texture) = self.recv_resource_load.try_recv() {
      let mut image = ggez::graphics::Image::from_rgba8(
        ctx,
        texture.width as u16,
        texture.height as u16,
        &texture.rgba_image,
      ).expect("could not create image from rgba");

      image.set_filter(ggez::graphics::FilterMode::Nearest);

      let mut ggez_image = texture
        .ggez_image
        .write()
        .expect("could not lock ggez_image");

      *ggez_image = Some(image);
    }
  }

  pub fn create_file(&self, path: &Path) -> io::Result<File> {
    File::create(self.path.join(path))
  }

  pub fn save<T: SaveableAsset>(&self, path: &Path, asset: &T) -> Result<(), Box<dyn Error>> {
    asset.save(self, path)
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

    Assets::new(path)
  }
}

pub trait Asset
where
  Self: Sized,
{
  fn load(assets: &Assets, path: &Path) -> Result<Self, Box<dyn Error>>;
}

pub trait SaveableAsset {
  fn save(&self, assets: &Assets, path: &Path) -> Result<(), Box<dyn Error>>;
}

impl<T> Asset for T
where
  for<'de> T: Deserialize<'de>,
{
  fn load(assets: &Assets, path: &Path) -> Result<Self, Box<dyn Error>> {
    let file = assets.open_file(path)?;

    Ok(serde_yaml::from_reader(file)?)
  }
}

impl<T: Serialize> SaveableAsset for T {
  fn save(&self, assets: &Assets, path: &Path) -> Result<(), Box<dyn Error>> {
    let file = assets.create_file(path)?;

    Ok(serde_yaml::to_writer(file, self)?)
  }
}
