// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `utils` module provides common utility functions and types.

pub mod ring;

mod droppable;

pub use self::droppable::Droppable;
pub use self::ring::Ring;
pub use quick_error::quick_error;
