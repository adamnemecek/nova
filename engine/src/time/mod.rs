// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod clock;
pub mod engine;

pub use self::clock::Clock;
pub use self::engine::Engine;

/// Number of times a `Clock` has ticked.
pub type Tick = u64;
