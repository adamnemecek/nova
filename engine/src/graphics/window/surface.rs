// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::backend;
use std::sync::Arc;

/// A rendering surface created from a [`Window`].
pub struct Surface {
  /// Raw backend surface structure.
  raw: backend::Surface,
  /// Reference to the backend instance the surface was created with. Stored so
  /// that the backend does not get dropped.
  _backend: Arc<backend::Instance>,
}

impl Surface {
  /// Creates a new surface from a window with the given backend instance.
  pub fn new(backend: &Arc<backend::Instance>, window: &winit::Window) -> Surface {
    let surface = backend.create_surface(&window);

    Surface {
      raw: surface,
      _backend: backend.clone(),
    }
  }
}

// Implement `AsMut` to expose a mutable reference to the raw backend surface.
impl AsMut<backend::Surface> for Surface {
  fn as_mut(&mut self) -> &mut backend::Surface {
    &mut self.raw
  }
}
