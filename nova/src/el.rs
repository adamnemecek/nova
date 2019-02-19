// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod spec;

mod context;
mod element;
mod hierarchy;
mod instance;
mod message;

pub use self::channels::ReceiveMessages;
pub use self::context::Context;
pub use self::element::{Element, ShouldRebuild};
pub use self::hierarchy::Hierarchy;
pub use self::message::{Message, MessageComposer, MessageQueue};
pub use self::spec::{spec, Spec};

use self::instance::Instance;
use crate::ecs;
use crate::engine::{self, Engine};

pub fn setup(engine: &mut Engine) {
  engine.resources_mut().insert(Hierarchy::new());
  engine.resources_mut().insert(MessageQueue::new());

  ecs::register::<hierarchy::Node>(engine.resources_mut());

  engine.on_event(
    engine::Event::ClockTimeUpdated,
    channels::DispatchReceiverMessages,
  );
}

pub fn print_all(res: &engine::Resources) {
  let hierarchy = res.fetch::<Hierarchy>();
  let nodes = ecs::read_components::<hierarchy::Node>(res);

  for entity in hierarchy.sorted.iter().cloned() {
    print!("\n{}: ", entity.id());

    if let Some(node) = nodes.get(entity) {
      print!("{:#?}, ", node.instance);

      print!(
        "spec_children: {:?}, ",
        node
          .spec_children
          .entities
          .iter()
          .map(|e| e.id())
          .collect::<Vec<_>>()
      );

      println!(
        "real_children: {:?}",
        node
          .real_children
          .entities
          .iter()
          .map(|e| e.id())
          .collect::<Vec<_>>()
      );
    }
  }
}

mod channels {
  use super::{Context, Element, MessageComposer, MessageQueue, ShouldRebuild};
  use crate::ecs;
  use crossbeam::channel;
  use std::fmt;

  #[derive(Debug)]
  pub struct ReceiveMessages<T> {
    pub receiver: channel::Receiver<T>,
    pub on_recv: MessageComposer<T>,
  }

  impl<T: fmt::Debug + Send + 'static> Element for ReceiveMessages<T> {
    type State = ();
    type Message = T;

    fn on_awake(&self, ctx: Context<Self>) {
      let on_recv = ctx.compose((), |_, msg| msg);

      ctx.put_component(MessageReceiver::new(self.receiver.clone(), on_recv));
    }

    fn on_change(&self, _: Self, ctx: Context<Self>) -> ShouldRebuild {
      self.on_awake(ctx);

      ShouldRebuild(false)
    }

    fn on_message(&self, msg: T, ctx: Context<Self>) -> ShouldRebuild {
      ctx.messages.send(self.on_recv.compose(msg));

      ShouldRebuild(false)
    }

    fn on_sleep(&self, ctx: Context<Self>) {
      ctx.remove_component::<MessageReceiver>();
    }
  }

  impl<T> PartialEq for ReceiveMessages<T> {
    fn eq(&self, _: &Self) -> bool {
      false // Cannot compare receivers.
    }
  }

  #[derive(Debug, Default)]
  pub struct DispatchReceiverMessages;

  impl<'a> ecs::System<'a> for DispatchReceiverMessages {
    type SystemData = (
      ecs::ReadResource<'a, MessageQueue>,
      ecs::ReadComponents<'a, MessageReceiver>,
    );

    fn run(&mut self, (queue, receivers): Self::SystemData) {
      use crate::ecs::Join;

      for receiver in (&receivers).join() {
        (receiver.receive)(&queue);
      }
    }
  }

  pub struct MessageReceiver {
    receive: Box<dyn Fn(&MessageQueue) + Send + Sync>,
  }

  impl MessageReceiver {
    pub fn new<T: Send + fmt::Debug + 'static>(
      receiver: channel::Receiver<T>,
      composer: MessageComposer<T>,
    ) -> Self {
      MessageReceiver {
        receive: Box::new(move |queue| {
          while let Ok(message) = receiver.try_recv() {
            queue.send(composer.compose(message));
          }
        }),
      }
    }
  }

  impl ecs::Component for MessageReceiver {
    type Storage = ecs::BTreeStorage<Self>;
  }

  impl fmt::Debug for MessageReceiver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "MessageReceiver")
    }
  }
}
