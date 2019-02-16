use super::Message;
use crate::ecs;
use crate::el::{Mount, MountContext, ShouldRebuild};
use crate::engine;
use crate::log;
use crossbeam::queue::SegQueue;

#[derive(Debug, Default)]
pub struct DeliveryQueue {
  pub(in crate::el) messages: SegQueue<Message>,
}

#[derive(Debug)]
pub struct DeliverMessages {
  log: log::Logger,
}

impl DeliverMessages {
  pub fn new() -> Self {
    DeliverMessages {
      log: log::Logger::new("nova::el::DeliverMsgs"),
    }
  }
}

impl Default for DeliverMessages {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a> ecs::System<'a> for DeliverMessages {
  type SystemData = (
    ecs::ReadResource<'a, DeliveryQueue>,
    ecs::WriteComponents<'a, Mount>,
  );

  fn setup(&mut self, res: &mut engine::Resources) {
    res.entry().or_insert_with(DeliveryQueue::default);
  }

  fn run(&mut self, (queue, mut mounts): Self::SystemData) {
    while let Ok(Message { recipient, payload }) = queue.messages.pop() {
      match mounts.get_mut(recipient) {
        Some(mount) => match mount.instance.on_message(
          payload,
          MountContext::new(recipient, &mount.node_children, &queue),
        ) {
          Ok(ShouldRebuild(true)) => {
            mount.needs_build = true;
          }

          Err(payload) => {
            self
              .log
              .debug("Message dropped.")
              .with("reason", "wrong type")
              .with("recipient", recipient.id())
              .with("payload", payload);
          }

          _ => {}
        },

        None => {
          self
            .log
            .debug("Message dropped.")
            .with("reason", "no mounted element")
            .with("recipient", recipient.id())
            .with("payload", payload);
        }
      }
    }
  }
}
