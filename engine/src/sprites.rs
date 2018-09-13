use ggez;
use serde_yaml;
use specs::prelude::*;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

pub use ggez::graphics::{Image, Rect};

/// Component representing a sprite to be rendered.
pub struct Sprite {
  /// Atlas to source frames from.
  pub atlas: Arc<Atlas>,
  /// Index of the frame in the atlas to render.
  pub frame: usize,
}

impl Component for Sprite {
  type Storage = HashMapStorage<Self>;
}

/// An image split into one or more frames. Also known as a spritesheet.
#[derive(Debug)]
pub struct Atlas {
  /// Image data for the atlas.
  pub image: Image,
  /// List of frames in the atlas, where each frame is a rectangular slice of
  /// the entire image.
  pub frames: Vec<Rect>,
}

impl Atlas {
  /// Creates a new atlas from the given `path`.
  pub fn new(ctx: &mut ggez::Context, path: impl Into<PathBuf>) -> Result<Atlas, Box<dyn Error>> {
    let mut path = path.into();

    // Append `.png` to the path and load it as the image.
    path.set_extension("png");

    let image = Image::new(ctx, &path)?;

    // Append `.yml` to the path and attempt to load it.
    path.set_extension("yml");

    if let Ok(file) = ggez::filesystem::open(ctx, path) {
      // Deserialize the file as `AtlasData`.
      let data = serde_yaml::from_reader::<_, AtlasData>(file)?;

      // Convert the data into an atlas with the loaded image.
      Ok(data.into_atlas(image))
    } else {
      // Return an atlas with one frame covering the entire loaded image.
      Ok(Atlas {
        image,
        frames: vec![Rect::new(0.0, 0.0, 1.0, 1.0)],
      })
    }
  }
}

/// Data loaded from a YAML file associated with an atlas.
#[derive(Debug, Deserialize)]
struct AtlasData {
  /// Number of columns in the grid that defines the frames of the atlas.
  columns: u16,
  /// Number of rows in the grid that defines the frames of the atlas.
  rows: u16,
}

impl AtlasData {
  /// Converts atlas data into an atlas by calculating the rects of each frame.
  fn into_atlas(self, image: Image) -> Atlas {
    let mut frames = Vec::with_capacity((self.columns * self.rows) as usize);

    let w = 1.0 / self.columns as f32;
    let h = 1.0 / self.rows as f32;

    for x in 0..self.columns {
      for y in 0..self.rows {
        let left = x as f32 * w;
        let top = y as f32 * h;

        frames.push(Rect::new(left, top, left + w, top + h));
      }
    }

    Atlas { image, frames }
  }
}
