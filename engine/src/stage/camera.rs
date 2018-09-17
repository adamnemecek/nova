// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Resource that stores the target of the stage's camera.
#[derive(Default)]
pub struct Camera {
  /// Current target of the camera.
  pub target: Target,
}

impl Camera {
  /// Sets the current target of the camera.
  pub fn set_target(&mut self, target: impl Into<Target>) {
    self.target = target.into();
  }
}

/// Target for a `Camera`.
pub enum Target {
  /// Fixes the camera at a particular position.
  Position(Point2<f32>),
  /// Follows an entity's position with the camera
  Entity(Entity),
}

// Set the default target to the origin point.
impl Default for Target {
  fn default() -> Self {
    Target::Position(Point2::new(0.0, 0.0))
  }
}

// Create targets from points.
impl From<Point2<f32>> for Target {
  fn from(point: Point2<f32>) -> Self {
    Target::Position(point)
  }
}

// Create targets from entities.
impl From<Entity> for Target {
  fn from(entity: Entity) -> Self {
    Target::Entity(entity)
  }
}
