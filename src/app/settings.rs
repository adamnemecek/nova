// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use serde_derive::*;

/// An error that occurred while parsing a TOML config file.
pub type TomlError = toml::de::Error;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings {
  #[serde(default)]
  pub window: window::Settings,
}

impl Settings {
  /// Parses configuration options from a string containing TOML.
  pub fn from_toml(source: &str) -> Result<Self, TomlError> {
    toml::from_str(source)
  }
}
