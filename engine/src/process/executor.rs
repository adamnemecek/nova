use super::atomic_wake::AtomicWake;
use super::{Process, Processes};
use crate::EngineHandle;
use std::sync::Arc;
use std::task::{self, LocalWaker, Poll};

pub struct Executor {
  engine: EngineHandle,
  items: Vec<Item>,
}

struct Item {
  process: Process,
  waker: Arc<AtomicWake>,
  local_waker: LocalWaker,
}

impl Executor {
  pub fn new(engine: &EngineHandle) -> Executor {
    engine.execute_mut(|ctx| {
      ctx.ensure_resource::<Processes>();
    });

    Executor {
      engine: engine.clone(),
      items: Vec::new(),
    }
  }

  pub fn tick(&mut self) {
    let items = &mut self.items;

    self.engine.execute_mut(|ctx| {
      let processes: &mut Processes = ctx.get_resource();

      for process in processes.pending.drain(..) {
        let waker = Arc::new(AtomicWake::new());

        items.push(Item {
          process,
          local_waker: task::local_waker_from_nonlocal(waker.clone()),
          waker,
        });
      }
    });

    self.items.drain_filter(|item| {
      if !item.waker.is_awake() {
        return false;
      }

      item.waker.reset();

      match item.process.0.as_mut().poll(&item.local_waker) {
        Poll::Ready(_) => true,
        Poll::Pending => false,
      }
    });
  }
}
