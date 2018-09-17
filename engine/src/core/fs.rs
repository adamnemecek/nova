// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

use serde::Deserialize;
use serde_yaml;
use std::error::Error;
use std::path::Path;

/// Load and deserialize the YAML file at the given `path` to a value fo type
/// `T`.
pub fn load_yaml<T>(core: &mut Core, path: &Path) -> Result<T, Box<dyn Error>>
where
  for<'de> T: Deserialize<'de>,
{
  let file = ggez::filesystem::open(&mut core.ctx, &path)?;

  Ok(serde_yaml::from_reader(file)?)
}

/// Load a ggez `Image` from the given `path`.
pub fn load_image(
  core: &mut Core,
  path: &Path,
  filter: ggez::graphics::FilterMode,
) -> Result<ggez::graphics::Image, Box<dyn Error>> {
  let mut image = ggez::graphics::Image::new(&mut core.ctx, path)?;

  image.set_filter(filter);

  Ok(image)
}
