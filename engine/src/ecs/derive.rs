// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `derive` module exports everything needed to use derive macros for
//! ECS types.
//!
//! It should be used like a prelude module:
//!
//!     use nova::ecs::derive::*;
//!

// TODO: Create a `nova-derive` crate with custom `Component` derive macro that
//   uses `::nova::ecs::Component` so that it doesn't need to be imported like
//   this. Also the macro should set the default storage to `HashMapStorage`.

pub use super::storages::*;
pub use super::Component;
pub use specs_derive::*;
