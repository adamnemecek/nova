// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use serde_yaml;
use std::error::Error;
use std::fs::File;
use std::path::Path;

/// Load and deserialize the YAML file at the given `path` to a value of type
/// `T`.
pub fn load<T>(path: &Path) -> Result<T, Box<dyn Error>>
where
  for<'de> T: Deserialize<'de>,
{
  let file = File::open(path)?;

  Ok(serde_yaml::from_reader(file)?)
}

/// Serialize a value to YAML and save it to the file at the given `path`.
pub fn save<T: Serialize>(path: &Path, value: &T) -> Result<(), Box<dyn Error>> {
  let file = File::create(path)?;

  Ok(serde_yaml::to_writer(file, value)?)
}
