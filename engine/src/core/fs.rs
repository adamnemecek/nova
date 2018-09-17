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
use std::path::{Path, PathBuf};

/// Provides access to files in the `assets` directory.
pub struct Assets {
  pub path: PathBuf,
}

impl Assets {
  /// Load and deserialize the YAML file at the given `path` to a value fo type
  /// `T`.
  pub fn load_yaml<T>(&self, path: &Path) -> Result<T, Box<dyn Error>>
  where
    for<'de> T: Deserialize<'de>,
  {
    load_yaml(&self.path.join(path))
  }

  /// Load a ggez `Image` from the given `path`.
  pub fn load_image(
    &self,
    ctx: &mut ggez::Context,
    path: &Path,
  ) -> Result<ggez::graphics::Image, Box<dyn Error>> {
    load_image(ctx, &self.path.join(path))
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

/// Load and deserialize the YAML file at the given `path` to a value of type
/// `T`.
pub fn load_yaml<T>(path: &Path) -> Result<T, Box<dyn Error>>
where
  for<'de> T: Deserialize<'de>,
{
  let file = File::open(path)?;

  Ok(serde_yaml::from_reader(file)?)
}

/// Serialize the value as YAML and save it to the file at the given `path`.
pub fn save_yaml<T: Serialize>(path: &Path, value: &T) -> Result<(), Box<dyn Error>> {
  let file = File::create(path)?;

  Ok(serde_yaml::to_writer(file, value)?)
}

/// Load a ggez `Image` from the given `path`.
pub fn load_image(
  ctx: &mut ggez::Context,
  path: &Path,
) -> Result<ggez::graphics::Image, Box<dyn Error>> {
  let img = {
    use std::io::Read;

    let mut buf = Vec::new();
    let mut file = File::open(path)?;

    file.read_to_end(&mut buf)?;
    image::load_from_memory(&buf)?.to_rgba()
  };

  let (width, height) = img.dimensions();
  let mut image = ggez::graphics::Image::from_rgba8(ctx, width as u16, height as u16, &img)?;

  image.set_filter(ggez::graphics::FilterMode::Nearest);

  Ok(image)
}
