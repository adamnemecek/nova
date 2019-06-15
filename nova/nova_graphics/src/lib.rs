// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod color;

pub use self::color::Color;

use nova_ecs as ecs;
use nova_math::{Matrix4, Point3, Rect};
use std::sync::Arc;

pub struct Image {}

pub struct Sprite {
  pub color: Color,
  pub image: Arc<Image>,
  pub rect: Rect<f32>,
}

pub struct Renderable {
  pub transform: Matrix4<f32>,
}

impl ecs::Component for Renderable {
  type Storage = ecs::HashMapStorage<Self>;
}

impl Renderable {
  pub fn position(&self) -> Point3<f32> {
    self.transform.transform_point(&Point3::origin())
  }
}

pub struct ScreenSpace {
  pub layer: i64,
  pub order: i64,
}
