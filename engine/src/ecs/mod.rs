// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `ecs` module exposes a parallel ECS implementation based on [specs][1].
//!
//! [1]: https://github.com/slide-rs/specs

pub mod components;
pub mod derive;
pub mod entities;
pub mod resources;
pub mod storages;
pub mod systems;

pub use self::components::*;
pub use self::entities::*;
pub use self::resources::*;
pub use self::storages::*;
pub use self::systems::*;
