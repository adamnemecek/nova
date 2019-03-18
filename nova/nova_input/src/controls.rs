// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod control;
mod update;

pub use self::control::Control;
pub use self::update::UpdateControls;

use crate::gamepad::{GamepadAxis, GamepadButton};
use crate::keyboard::KeyCode;
use crate::mouse::MouseButton;
use nova_core::collections::{FnvHashMap, FnvHashSet};
use nova_core::ecs;
use nova_core::engine::Engine;
use nova_core::events;
use nova_core::log::warn;
use nova_core::SharedStr;
use std::f32;
use std::mem;

pub type ReadControls<'a> = ecs::ReadResource<'a, Controls>;
pub type WriteControls<'a> = ecs::WriteResource<'a, Controls>;

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
        .single_write(dbg!(ControlEvent::Changed { id, value }));

      if state.is_pressed {
        if abs_value < 0.65 {
          state.is_pressed = false;

          self
            .events
            .single_write(dbg!(ControlEvent::Released { id }));
        }
      } else if abs_value >= 0.75 {
        state.is_pressed = true;

        self.events.single_write(dbg!(ControlEvent::Pressed { id }));
      }
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ControlId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlBinding {
  Key(KeyCode),
  MouseButton(MouseButton),
  GamepadButton(GamepadButton),
  GamepadAxis(GamepadAxis),
}

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

pub fn read(res: &ecs::Resources) -> ReadControls {
  ecs::SystemData::fetch(res)
}

pub fn write(res: &ecs::Resources) -> WriteControls {
  ecs::SystemData::fetch(res)
}
