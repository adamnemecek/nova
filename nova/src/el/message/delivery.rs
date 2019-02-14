use super::Message;
use crate::ecs;
use crate::el::{Mount, RebuildRequired, ShouldRebuild};
use crate::engine;
use crate::log;
use crossbeam::queue::SegQueue;

#[derive(Debug, Default)]
pub struct DeliveryQueue {
  pub(in crate::el) messages: SegQueue<Message>,
}

#[derive(Debug)]
pub struct DeliverMessages {
  msg_stack: Vec<Message>,
  log: log::Logger,
}

impl DeliverMessages {
  pub fn new() -> Self {
    DeliverMessages {
      msg_stack: Vec::with_capacity(1000),
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
    ecs::WriteComponents<'a, RebuildRequired>,
  );

  fn setup(&mut self, res: &mut engine::Resources) {
    res.entry().or_insert_with(DeliveryQueue::default);
  }

  fn run(&mut self, (queue, mut mounts, mut needs_rebuild): Self::SystemData) {
    while let Ok(msg) = queue.messages.pop() {
      self.msg_stack.push(msg);

      while let Some(Message { recipient, payload }) = self.msg_stack.pop() {
        match mounts.get_mut(recipient) {
          Some(mount) => match mount.instance.on_message(payload, &queue) {
            Ok(ShouldRebuild(true)) => {
              let _ = needs_rebuild.insert(recipient, RebuildRequired);
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
}
