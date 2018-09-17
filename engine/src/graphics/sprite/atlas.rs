// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

/// Coordinates for a cell in an atlas.
pub type Cell = (usize, usize);

/// An image split into one or more cells.
///
/// Also known as a spritesheet.
pub struct Atlas {
  /// Image to render cells from.
  pub image: ggez::graphics::Image,
  /// Data describing the atlas.
  pub data: Data,
}

impl Atlas {
  /// Gets the source rectangle for a given `cell` in the atlas.
  pub fn get(&self, cell: Cell) -> ggez::graphics::Rect {
    let w = self.data.cell_width as f32 / self.image.width() as f32;
    let h = self.data.cell_height as f32 / self.image.height() as f32;

    let x = cell.0 as f32 * w;
    let y = cell.1 as f32 * h;

    ggez::graphics::Rect::new(x, y, w, h)
  }
}

/// Data for an atlas.
#[derive(Serialize, Deserialize)]
pub struct Data {
  /// Width of a single cell in the atlas.
  pub cell_width: usize,
  /// Height of a single cell in the atlas.
  pub cell_height: usize,
}
