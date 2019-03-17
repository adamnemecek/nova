// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::elements::{Element, MessageHandler};
use crate::nodes::NodeContext;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct ElementContext<'a, E: Element> {
  pub state: &'a mut E::State,
  pub(crate) message_handlers: &'a mut HashMap<TypeId, MessageHandler<E>>,
  pub(crate) node: NodeContext<'a>,
}

impl<'a, E: Element> ElementContext<'a, E> {
  pub fn subscribe<M, H>(&mut self, mut handler: H)
  where
    M: Any + Send + Sync,
    H: for<'b> FnMut(&E, ElementContext<'b, E>, M) + Send + Sync + 'static,
  {
    self.message_handlers.insert(
      TypeId::of::<M>(),
      Box::new(move |element, ctx, payload| {
        let payload = payload.downcast::<M>()?;

        match Arc::try_unwrap(payload) {
          Ok(payload) => {
            handler(element, ctx, payload);

            Ok(())
          }

          Err(payload) => Err(payload),
        }
      }),
    );
  }
}

impl<'a, E: Element> Deref for ElementContext<'a, E> {
  type Target = NodeContext<'a>;

  fn deref(&self) -> &Self::Target {
    &self.node
  }
}

impl<'a, E: Element> DerefMut for ElementContext<'a, E> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.node
  }
}
