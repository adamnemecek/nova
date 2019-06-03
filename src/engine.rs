// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::systems::{System, SystemDispatcher};
use nova::components;
use nova::crossbeam::channel;
use nova::entities;
use nova::shred::{Resource, Resources, SystemData};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

pub struct Engine {
  resources: Resources,
  systems: HashMap<TypeId, Box<dyn Any + Send>>,
}

type SystemRunner = Box<dyn FnMut(&dyn Any, &Resources) + Send>;

impl Engine {
  pub fn new() -> EngineHandle {
    let mut resources = Resources::new();

    entities::set_up(&mut resources);
    components::set_up(&mut resources);

    let (sender, receiver) = channel::unbounded();

    let handle = EngineHandle {
      engine: Arc::new(Mutex::new(Engine {
        resources,
        systems: Default::default(),
      })),
      channel: sender,
    };

    let result = handle.clone();

    thread::spawn(move || {
      for message in receiver {
        handle.lock().receive(message);
      }
    });

    result
  }

  pub fn put_resource<T: Resource>(&mut self, resource: T) -> &mut Self {
    self.resources.insert(resource);
    self
  }

  pub fn add_system<T: 'static>(
    &mut self,
    system: impl for<'a> System<'a, T> + Send + 'static,
  ) -> &mut Self {
    let dispatcher = self
      .systems
      .entry(TypeId::of::<T>())
      .or_insert_with(|| Box::new(SystemDispatcher::<T>::default()));

    let dispatcher = dispatcher
      .downcast_mut::<SystemDispatcher<T>>()
      .expect("failed to downcast_mut dispatcher");

    dispatcher.add(system);

    self
  }

  pub fn dispatch<T: 'static>(&mut self, message: T) -> &mut Self {
    let dispatcher = self
      .systems
      .get_mut(&TypeId::of::<T>())
      .and_then(|d| d.downcast_mut::<SystemDispatcher<T>>());

    if let Some(dispatcher) = dispatcher {
      dispatcher.run(&message, &self.resources);
    }

    self
  }

  pub fn execute(&mut self, func: impl FnOnce(&mut Engine)) -> &mut Self {
    func(self);

    self
  }

  fn receive(&mut self, message: EngineMessage) {
    match message {
      EngineMessage::ExecuteFunction(func) => func(self),
    }
  }
}

pub enum EngineMessage {
  ExecuteFunction(Box<dyn FnOnce(&mut Engine) + Send>),
}

#[derive(Clone)]
pub struct EngineHandle {
  engine: Arc<Mutex<Engine>>,
  channel: channel::Sender<EngineMessage>,
}

impl EngineHandle {
  pub fn lock(&self) -> MutexGuard<Engine> {
    self.engine.lock().expect("failed to lock engine")
  }

  pub fn execute(&self, func: impl FnOnce(&mut Engine) + Send + 'static) {
    self
      .channel
      .send(EngineMessage::ExecuteFunction(Box::new(func)))
      .expect("failed to send ExecuteFunction message to engine")
  }

  pub fn dispatch(&self, message: impl Send + 'static) {
    self.execute(move |engine| {
      engine.dispatch(message);
    });
  }
}
