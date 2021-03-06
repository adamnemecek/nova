// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod context;
mod element;
mod instance;
mod prototype;
mod state;

pub use self::context::ElementContext;
pub use self::element::Element;
pub use self::state::ElementState;

pub use self::instance::ElementInstance;
pub use self::prototype::ElementPrototype;

use crate::messages::MessagePayload;

type MessageHandler<E> =
  Box<dyn FnMut(&E, ElementContext<E>, MessagePayload) -> Result<(), MessagePayload> + Send + Sync>;
