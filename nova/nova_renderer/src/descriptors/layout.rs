// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::DescriptorKind;
use crate::{Backend, Device, DeviceExt};
use std::sync::Arc;

pub type RawDescriptorLayout = <Backend as gfx_hal::Backend>::DescriptorSetLayout;

#[derive(Debug, Clone)]
pub struct DescriptorLayout {
  inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
  raw: RawDescriptorLayout,
  kinds: Vec<DescriptorKind>,
}

impl DescriptorLayout {
  pub fn new(device: &Device, kinds: Vec<DescriptorKind>) -> Self {
    let bindings = kinds.iter().enumerate().map(|(index, kind)| {
      gfx_hal::pso::DescriptorSetLayoutBinding {
        binding: index as u32,
        ty: kind.ty(),
        count: kind.count(),
        stage_flags: kind.stage_flags(),
        immutable_samplers: false,
      }
    });

    let raw = unsafe {
      device
        .create_descriptor_set_layout(bindings, &[])
        .expect("Could not create descriptor layout")
    };

    DescriptorLayout {
      inner: Arc::new(Inner { raw, kinds }),
    }
  }

  pub(crate) fn raw(&self) -> &RawDescriptorLayout {
    &self.inner.raw
  }

  pub(crate) fn kinds(&self) -> std::slice::Iter<DescriptorKind> {
    self.inner.kinds.iter()
  }

  pub fn destroy(self, device: &Device) {
    if let Ok(inner) = Arc::try_unwrap(self.inner) {
      unsafe {
        device.destroy_descriptor_set_layout(inner.raw);
      }
    }
  }
}
