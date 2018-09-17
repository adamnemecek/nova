// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

pub mod animation;
pub mod template;

pub use self::animation::{Animation, AnimationSystem};
pub use self::template::ObjectTemplate;

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Object {
  pub template: Arc<ObjectTemplate>,
  pub facing: Vector3<f32>,
}
