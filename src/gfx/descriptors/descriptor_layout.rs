// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A layout defining [`Descriptor`]s bound to a [`DescriptorSet`].
pub struct DescriptorLayout {
  context: Arc<Context>,
  layout: Expect<backend::DescriptorLayout>,
  kinds: Vec<DescriptorKind>,
}

impl DescriptorLayout {
  /// Creates a new descriptor layout with bindings for the given kinds of
  /// descriptor.
  pub fn new(
    context: &Arc<Context>,
    kinds: impl Into<Vec<DescriptorKind>>,
  ) -> Result<Self, OutOfMemoryError> {
    let kinds = kinds.into();

    let bindings =
      kinds.iter().enumerate().map(|(index, kind)| gfx_hal::pso::DescriptorSetLayoutBinding {
        binding: index as u32,
        ty: kind.backend_ty(),
        count: 1,
        stage_flags: gfx_hal::pso::ShaderStageFlags::ALL,
        immutable_samplers: false,
      });

    let layout = unsafe { context.device().create_descriptor_set_layout(bindings, &[])? };

    Ok(Self { context: context.clone(), layout: layout.into(), kinds })
  }

  /// Returns a reference to the graphics context the descriptor layout was
  /// created in.
  pub fn context(&self) -> &Arc<Context> {
    &self.context
  }

  /// Returns a reference to the kinds of descriptors defined in the layout, in
  /// binding order.
  pub fn kinds(&self) -> &[DescriptorKind] {
    &self.kinds
  }

  /// Returns a reference to the underlying backend descriptor layout.
  pub fn as_backend(&self) -> &backend::DescriptorLayout {
    &self.layout
  }
}

// Implement `Drop` to clean up device resources.
impl Drop for DescriptorLayout {
  fn drop(&mut self) {
    unsafe {
      self.context.device().destroy_descriptor_set_layout(self.layout.take());
    }
  }
}
