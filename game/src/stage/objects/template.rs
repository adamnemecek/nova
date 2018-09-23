// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Animation, AnimationData};
use crate::prelude::*;
use nova::assets;
use nova::graphics::Atlas;
use std::sync::Arc;

/// Template for an object on the stage.
#[derive(Debug)]
pub struct Template {
  /// Atlas for the object's sprite.
  pub atlas: Arc<Atlas>,
  /// Whether the object only faces cardinal directions, rather than all eight
  /// compass directions.
  pub cardinal_dirs_only: bool,
  /// Size of the object's shadow.
  pub shadow_size: Vector2<f32>,
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
impl assets::Asset for Template {
  fn load(assets: &assets::OverlayFs, path: &assets::Path) -> Result<Self, assets::Error> {
    let mut path = path.to_owned();

    let data = assets.load::<TemplateData>(&path)?;

    path.pop();

    let atlas = assets.load(&path.join(&data.atlas))?;

    Ok(Template {
      atlas: Arc::new(atlas),
      cardinal_dirs_only: data.cardinal_dirs_only,
      shadow_size: Vector2::new(data.shadow_size.0, data.shadow_size.1),
      animations: data.animations.into_iter().map(Animation::from).collect(),
    })
  }
}

/// Data for an `ObjectTemplate`.
#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateData {
  /// Path to the atlas for the object's sprite.
  pub atlas: assets::PathBuf,
  /// List of data for the object's animations.
  pub animations: Vec<AnimationData>,
  /// Whether the object only faces cardinal directions, rather than all eight
  /// compass directions.
  #[serde(default)]
  pub cardinal_dirs_only: bool,
  /// Size of the object's shadow in pixels.
  #[serde(default)]
  pub shadow_size: (f32, f32),
}
