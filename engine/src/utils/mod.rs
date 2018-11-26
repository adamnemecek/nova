// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `utils` module provides common utility functions and types.

extern crate lazy_static;
extern crate quick_error;

mod droppable;
mod ring;

pub use self::droppable::*;
pub use self::ring::*;
pub use lazy_static::lazy_static;
pub use quick_error::quick_error;
