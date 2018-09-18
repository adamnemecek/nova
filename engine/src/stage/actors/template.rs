// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use std::error::Error;
use std::path::{Path, PathBuf};

/// Names of the object animations to use for each `Mode` in order.
pub const MODE_ANIMATION_NAMES: [&'static str; MODE_COUNT] = ["none", "idle", "walk"];

/// Template for an actor.
#[derive(Debug)]
pub struct Template {
  /// Template for the actor's object.
  pub object: Arc<objects::Template>,
  /// Speed the actor walks in pixels per second.
  pub walk_speed: f32,
  /// List of object animation indices for each `Mode` in order.
  pub mode_animations: [usize; MODE_COUNT],
}

// Support loading templates from asset files.
impl core::Asset for Template {
  fn load(assets: &core::Assets, path: &Path) -> Result<Self, Box<dyn Error>> {
    let mut path = path.to_owned();
    let data = assets.load::<TemplateData>(&path)?;

    path.pop();

    let object = assets.load(&path.join(data.object))?;

    let mut template = Template {
      object: Arc::new(object),
      walk_speed: data.walk_speed,
      mode_animations: Default::default(),
    };

    for (i, animation_name) in MODE_ANIMATION_NAMES.iter().enumerate() {
      template.mode_animations[i] = template.object.find_animation(animation_name);
    }

    Ok(template)
  }
}

/// Data for an actor template.
#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateData {
  /// Path to the actor's object template.
  pub object: PathBuf,
  /// Speed the actor walks in pixels per second.
  pub walk_speed: f32,
}
