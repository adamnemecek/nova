// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use std::error::Error;
use std::path::{Path, PathBuf};

/// Template for an object on the stage.
#[derive(Debug)]
pub struct Template {
  /// Atlas for the object's sprite.
  pub atlas: Arc<graphics::Atlas>,
  /// Whether the object only faces cardinal directions, rather than all eight
  /// compass directions.
  pub cardinal_dirs_only: bool,
  /// Size of the object's shadow.
  pub shadow_size: (f32, f32),
  /// List of animations supported by the object.
  pub animations: Vec<Animation>,
}

impl Template {
  /// Finds the index of the animation with the given `name`.
  ///
  /// Returns `0` if the animation was not found.
  pub fn find_animation(&self, name: impl AsRef<str>) -> usize {
    let name = name.as_ref();

    self
      .animations
      .iter()
      .position(|a| a.name == name)
      .unwrap_or_default()
  }
}

// Support loading object templates as assets.
impl core::Asset for Template {
  fn load(assets: &core::Assets, path: &Path) -> Result<Self, Box<dyn Error>> {
    let mut path = path.to_owned();

    let data = assets.load::<TemplateData>(&path)?;

    path.pop();

    let atlas = assets.load(&path.join(&data.atlas))?;

    Ok(Template {
      atlas: Arc::new(atlas),
      cardinal_dirs_only: data.cardinal_dirs_only,
      shadow_size: data.shadow_size,
      animations: data.animations.into_iter().map(Animation::from).collect(),
    })
  }
}

/// Data for an `ObjectTemplate`.
#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateData {
  /// Path to the atlas for the object's sprite.
  pub atlas: PathBuf,
  /// List of data for the object's animations.
  pub animations: Vec<animation::AnimationData>,
  /// Whether the object only faces cardinal directions, rather than all eight
  /// compass directions.
  #[serde(default)]
  pub cardinal_dirs_only: bool,
  /// Size of the object's shadow in pixels.
  #[serde(default)]
  pub shadow_size: (f32, f32),
}
