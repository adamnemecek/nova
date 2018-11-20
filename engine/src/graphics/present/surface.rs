// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::backend;
use crate::math::Size;
use crate::window::{self, Window};
use std::sync::Arc;

/// A rendering surface created from a [`Window`].
pub struct Surface {
  /// Raw backend surface structure.
  raw: backend::Surface,
  /// Handle to the window the surface was created from.
  window: window::Handle,
  /// Reference to the backend instance the surface was created with.
  backend: Arc<backend::Instance>,
}

impl Surface {
  /// Creates a new surface from a window with the given backend instance.
  pub fn new(backend: &Arc<backend::Instance>, window: &Window) -> Surface {
    let surface = backend.create_surface(window.handle());

    Surface {
      raw: surface,
      window: window.handle().clone(),
      backend: backend.clone(),
    }
  }

  /// Gets a reference to tho backend instance the surface was created with.
  pub fn backend(&self) -> &Arc<backend::Instance> {
    &self.backend
  }

  /// Determines the current size of the surface in pixels.
  pub fn calculate_size(&self) -> Size<u32> {
    self.window.calculate_inner_size()
  }
}

// Implement `AsRef` and `AsMut` to expose the raw backend surface.
impl AsRef<backend::Surface> for Surface {
  fn as_ref(&self) -> &backend::Surface {
    &self.raw
  }
}

impl AsMut<backend::Surface> for Surface {
  fn as_mut(&mut self) -> &mut backend::Surface {
    &mut self.raw
  }
}
