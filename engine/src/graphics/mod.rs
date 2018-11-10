// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod device;
pub mod hal;
pub mod image;
pub mod rendering;
pub mod window;

mod color;

pub use self::color::*;
pub use self::device::Device;
pub use self::image::Image;
pub use self::window::Window;

use std::sync::Arc;

pub fn create_backend(app_name: &str, app_version: u32) -> Arc<hal::backend::Instance> {
  Arc::new(hal::backend::Instance::create(app_name, app_version))
}
