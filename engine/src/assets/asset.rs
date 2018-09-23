// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Error, OverlayFs, Path};
use serde::{Deserialize, Serialize};
use serde_yaml;

/// Trait for types that can be loaded from a file.
pub trait Asset: Sized {
  /// Loads an asset from the given path in the given VFS.
  fn load(fs: &OverlayFs, path: &Path) -> Result<Self, Error>;
}

/// Trait for types that can be saved to a file.
pub trait SaveableAsset {
  /// Saves the asset to the given path in the given VFS.
  fn save(&self, fs: &OverlayFs, path: &Path) -> Result<(), Error>;
}

// Implements `Asset` for serde-deserializable types by loading YAML.
impl<T> Asset for T
where
  for<'de> T: Deserialize<'de>,
{
  fn load(fs: &OverlayFs, path: &Path) -> Result<Self, Error> {
    let file = fs.open(path)?;

    Ok(serde_yaml::from_reader(file)?)
  }
}

// Implements `SaveableAsset` for serde-serializable types by saving YAML.
impl<T: Serialize> SaveableAsset for T {
  fn save(&self, fs: &OverlayFs, path: &Path) -> Result<(), Error> {
    let file = fs.create(path)?;

    Ok(serde_yaml::to_writer(file, self)?)
  }
}
