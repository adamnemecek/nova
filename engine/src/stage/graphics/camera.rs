// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Resource that stores the target of the stage's camera.
#[derive(Default)]
pub struct Camera {
  /// Current target of the camera.
  pub target: CameraTarget,
}

impl Camera {
  /// Sets the current target of the camera.
  pub fn set_target(&mut self, target: impl Into<CameraTarget>) {
    self.target = target.into();
  }
}

/// Target for a `Camera`.
pub enum CameraTarget {
  /// Fixes the camera at a particular position.
  Position(Point2<f32>),
  /// Follows an entity's position with the camera
  Entity(Entity),
}

// Set the default target to the origin point.
impl Default for CameraTarget {
  fn default() -> Self {
    CameraTarget::Position(Point2::new(0.0, 0.0))
  }
}

// Create targets from points.
impl From<Point2<f32>> for CameraTarget {
  fn from(point: Point2<f32>) -> Self {
    CameraTarget::Position(point)
  }
}

// Create targets from entities.
impl From<Entity> for CameraTarget {
  fn from(entity: Entity) -> Self {
    CameraTarget::Entity(entity)
  }
}

/// Sets the target of the stage camera in the given engine context.
pub fn set_camera_target(ctx: &mut engine::Context, target: impl Into<CameraTarget>) {
  engine::fetch_resource_mut::<Camera>(ctx).set_target(target);
}
