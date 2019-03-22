// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod map;

mod binding;
mod control;
mod update;

pub use self::binding::ControlBinding;
pub use self::control::Control;
pub use self::map::ControlMap;
pub use self::update::UpdateControls;

use nova_core::collections::{FnvHashMap, FnvHashSet};
use nova_core::engine::Engine;
use nova_core::events;
use nova_core::log::warn;
use nova_core::resources::{self, ReadResource, Resources, WriteResource};
use nova_core::SharedStr;
use std::f32;
use std::mem;

pub type ReadControls<'a> = ReadResource<'a, Controls>;
pub type WriteControls<'a> = WriteResource<'a, Controls>;

#[derive(Default)]
pub struct Controls {
  pub events: events::Channel<ControlEvent>,
  states: Vec<Control>,
  by_name: FnvHashMap<SharedStr, ControlId>,
  by_binding: FnvHashMap<ControlBinding, FnvHashSet<ControlId>>,
}

impl Controls {
  pub fn add(&mut self, name: impl Into<SharedStr>) -> ControlId {
    let id = ControlId(self.states.len());
    let name = name.into();

    if self.by_name.insert(name.clone(), id).is_some() {
      warn!("Duplicate control name {:?}.", name);
    }

    self.states.push(Control {
      name,
      value: 0.0,
      is_pressed: false,
      bindings: Default::default(),
      negative_bindings: Default::default(),
    });

    id
  }

  pub fn lookup(&self, name: &str) -> Option<ControlId> {
    self.by_name.get(name).cloned()
  }

  pub fn lookup_or_add(&mut self, name: impl AsRef<str> + Into<SharedStr>) -> ControlId {
    match self.lookup(name.as_ref()) {
      Some(id) => id,
      None => self.add(name),
    }
  }

  pub fn bind(&mut self, id: ControlId, binding: ControlBinding) {
    let state = &mut self.states[id.0];

    state.negative_bindings.remove(&binding);
    state.bindings.insert(binding);

    self.by_binding.entry(binding).or_default().insert(id);
  }

  pub fn bind_negative(&mut self, id: ControlId, input: ControlBinding) {
    let state = &mut self.states[id.0];

    state.bindings.remove(&input);
    state.negative_bindings.insert(input);

    self.by_binding.entry(input).or_default().insert(id);
  }

  pub fn unbind(&mut self, id: ControlId, input: ControlBinding) {
    let state = &mut self.states[id.0];

    state.bindings.remove(&input);
    state.negative_bindings.remove(&input);

    if let Some(ids) = self.by_binding.get_mut(&input) {
      ids.remove(&id);
    }
  }

  pub fn clear_bindings(&mut self) {
    for state in &mut self.states {
      state.bindings.clear();
      state.negative_bindings.clear();
    }

    self.by_binding.clear();
  }

  pub fn apply_bindings(&mut self, map: &ControlMap) {
    for (name, bindings) in &map.bindings {
      let id = self.lookup_or_add(name);

      for binding in &bindings.positive {
        self.bind(id, *binding);
      }

      for binding in &bindings.negative {
        self.bind_negative(id, *binding);
      }
    }
  }

  pub fn get(&self, id: ControlId) -> &Control {
    &self.states[id.0]
  }

  pub fn set_bound_values(&mut self, binding: ControlBinding, value: f32) {
    let bound = match self.by_binding.get(&binding) {
      Some(bound) => bound,
      None => return,
    };

    for id in bound.iter().cloned() {
      let state = &mut self.states[id.0];

      let value = if state.negative_bindings.contains(&binding) {
        -value
      } else {
        value
      };

      let old_value = mem::replace(&mut state.value, value);

      if (old_value - value).abs() <= f32::EPSILON {
        return;
      }

      let abs_value = value.abs();

      self
        .events
        .single_write(ControlEvent::Changed { id, value });

      if state.is_pressed {
        if abs_value < 0.65 {
          state.is_pressed = false;

          self.events.single_write(ControlEvent::Released { id });
        }
      } else if abs_value >= 0.75 {
        state.is_pressed = true;

        self.events.single_write(ControlEvent::Pressed { id });
      }
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ControlId(pub(crate) usize);

#[derive(Debug)]
pub enum ControlEvent {
  Changed { id: ControlId, value: f32 },
  Pressed { id: ControlId },
  Released { id: ControlId },
}

pub fn setup(engine: &mut Engine) {
  engine.resources.insert(Controls::default());

  update::setup(engine);
}

pub fn borrow(res: &Resources) -> ReadControls {
  resources::borrow(res)
}

pub fn borrow_mut(res: &Resources) -> WriteControls {
  resources::borrow_mut(res)
}
