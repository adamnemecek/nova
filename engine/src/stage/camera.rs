// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

#[derive(Default)]
pub struct Camera {
  pub target: Target,
}

impl Camera {
  pub fn set_target(&mut self, target: impl Into<Target>) {
    self.target = target.into();
  }
}

pub enum Target {
  Position(Point2<f32>),
  Entity(Entity),
}

impl Default for Target {
  fn default() -> Self {
    Target::Position(Point2::new(0.0, 0.0))
  }
}

impl From<Point2<f32>> for Target {
  fn from(point: Point2<f32>) -> Self {
    Target::Position(point)
  }
}

impl From<Entity> for Target {
  fn from(entity: Entity) -> Self {
    Target::Entity(entity)
  }
}
