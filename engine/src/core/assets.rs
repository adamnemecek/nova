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

/// Provides access to files in an assets directory, typically the game's
/// `assets` folder.
pub struct Assets {
  /// Path to the assets directory..
  pub path: PathBuf,
  /// Sender for queueing resources to load on the main thread with access to
  /// the `ggez::Context`.
  send_resource_load: crossbeam_channel::Sender<Texture>,
  /// Receiver for queued resources to load on the main thread with access to
  /// the `ggez::Context`.
  recv_resource_load: crossbeam_channel::Receiver<Texture>,
}

impl Assets {
  /// Creates a new `Assets` for the asset directory at the given `path`.
  pub fn new(path: impl Into<PathBuf>) -> Self {
    let (send_resource_load, recv_resource_load) = crossbeam_channel::unbounded();

    Assets {
      path: path.into(),
      send_resource_load,
      recv_resource_load,
    }
  }

  /// Opens the file at the given `path` relative to the asset directory.
  pub fn open_file(&self, path: &Path) -> io::Result<File> {
    File::open(self.path.join(path))
  }

  /// Loads the asset at the given `path` relative to the asset directory.
  pub fn load<T: Asset>(&self, path: &Path) -> Result<T, Box<dyn Error>> {
    T::load(self, path)
  }

  /// Saves an asset to the given `path` relative to the asset directory.
  pub fn save<T: SaveableAsset>(&self, path: &Path, asset: &T) -> Result<(), Box<dyn Error>> {
    asset.save(self, path)
  }

  /// Queues a resource to load on the main thread with access to the
  /// `ggez::Context``.
  pub fn queue_resource_load(&self, texture: Texture) {
    self.send_resource_load.send(texture);
  }

  /// Loads queued resources that need access to the `ggez::Context`.
  pub fn load_queued_resources(&self, ctx: &mut ggez::Context) {
    // For each queued texture…
    while let Some(texture) = self.recv_resource_load.try_recv() {
      // Create a ggez image from the RGBA8 image data.
      let mut image = ggez::graphics::Image::from_rgba8(
        ctx,
        texture.width as u16,
        texture.height as u16,
        &texture.rgba_image,
      ).expect("could not create image from rgba");

      // Set filter to nearest to scale sprites cleanly and prevent artifacts
      // when scaling PNGs with an alpha channel.
      image.set_filter(ggez::graphics::FilterMode::Nearest);

      // Update the texture with the image by acquiring a write lock.
      let mut ggez_image = texture
        .ggez_image
        .write()
        .expect("could not lock ggez_image");

      *ggez_image = Some(image);
    }
  }

  /// Creates a file at the given `path` relative to the asset directory.
  pub fn create_file(&self, path: &Path) -> io::Result<File> {
    File::create(self.path.join(path))
  }
}

impl Default for Assets {
  fn default() -> Self {
    // If `CARGO_MANIFEST_DIR` is set, use the `assets` directory from the
    // project's directory.
    let mut path = env::var("CARGO_MANIFEST_DIR")
      .map(PathBuf::from)
      .unwrap_or_else(|_| {
        // Otherwise use the `assets` directory in the exe's directory.
        let mut path = env::current_exe().expect("could not get current exe path");

        path.pop();
        path
      });

    path.push("assets");

    Assets::new(path)
  }
}

/// Trait for types that represent assets that can be loaded from the `assets`
/// directory.
pub trait Asset
where
  Self: Sized,
{
  /// Loads an asset from the given `path`.
  fn load(assets: &Assets, path: &Path) -> Result<Self, Box<dyn Error>>;
}

/// Trait for types that represent assets that can be saved to the `assets`
/// directory.
pub trait SaveableAsset {
  /// Saves the asset to the given `path`.
  fn save(&self, assets: &Assets, path: &Path) -> Result<(), Box<dyn Error>>;
}

// Implements `Asset` for serde-deserializable types by loading YAML.
impl<T> Asset for T
where
  for<'de> T: Deserialize<'de>,
{
  fn load(assets: &Assets, path: &Path) -> Result<Self, Box<dyn Error>> {
    let file = assets.open_file(path)?;

    Ok(serde_yaml::from_reader(file)?)
  }
}

// Implements `SaveableAsset` for serde-serializable types by saving YAML.
impl<T: Serialize> SaveableAsset for T {
  fn save(&self, assets: &Assets, path: &Path) -> Result<(), Box<dyn Error>> {
    let file = assets.create_file(path)?;

    Ok(serde_yaml::to_writer(file, self)?)
  }
}
