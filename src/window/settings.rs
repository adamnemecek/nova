// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use std::env;

lazy_static! {
  static ref DEFAULT_TITLE: String = {
    env::current_exe()
      .ok()
      .and_then(|exe| Some(exe.file_stem()?.to_string_lossy().into()))
      .unwrap_or_else(String::new)
  };
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Settings {
  #[serde(default)]
  pub title: Option<String>,
  #[serde(default)]
  pub resizable: Option<bool>,
  #[serde(default)]
  pub size: Option<Size<f64>>,
}

pub(crate) fn default_title() -> String {
  DEFAULT_TITLE.clone()
}
