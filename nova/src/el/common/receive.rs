use crate::ecs;
use crate::el;
use crate::engine::{self, Engine};
use crossbeam::channel;
use std::fmt;

#[derive(Debug)]
pub struct Receive<T> {
  pub receiver: channel::Receiver<T>,
  pub on_recv: el::MessageComposer<T>,
}

impl<T: fmt::Debug + Send + 'static> el::Element for Receive<T> {
  type State = ();
  type Message = T;

  fn on_awake(&self, ctx: el::Context<Self>) {
    let on_recv = ctx.compose((), |_, msg| msg);

    ctx.put_component(MessageReceiver::new(self.receiver.clone(), on_recv));
  }

  fn on_change(&self, _: Self, ctx: el::Context<Self>) -> el::ShouldRebuild {
    self.on_awake(ctx);

    el::ShouldRebuild(false)
  }

  fn on_message(&self, msg: T, ctx: el::Context<Self>) -> el::ShouldRebuild {
    ctx.messages.send(self.on_recv.compose(msg));

    el::ShouldRebuild(false)
  }

  fn on_sleep(&self, ctx: el::Context<Self>) {
    ctx.remove_component::<MessageReceiver>();
  }
}

impl<T> PartialEq for Receive<T> {
  fn eq(&self, _: &Self) -> bool {
    false // Cannot compare receivers.
  }
}

#[derive(Debug, Default)]
struct ReceiveMessages;

impl<'a> ecs::System<'a> for ReceiveMessages {
  type SystemData = (
    ecs::ReadResource<'a, el::MessageQueue>,
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
  receive: Box<dyn Fn(&el::MessageQueue) + Send + Sync>,
}

impl MessageReceiver {
  pub fn new<T: Send + fmt::Debug + 'static>(
    receiver: channel::Receiver<T>,
    composer: el::MessageComposer<T>,
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

pub fn setup(engine: &mut Engine) {
  engine.on_event(engine::Event::ClockTimeUpdated, ReceiveMessages);
}
