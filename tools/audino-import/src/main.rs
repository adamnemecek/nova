// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate clap;
extern crate nova_engine;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

use clap::{App, Arg};
use nova_engine::prelude::*;
use std::fs;
use std::path::PathBuf;

mod animations;

/// Main entry point for the tool.
fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Create a clap app to parse arguments.
  let app = App::new("audino-import")
    .about("Imports sprite sheets and animations from Audino's PMD animation dump.")
    .arg(
      Arg::with_name("src")
        .help("Path to a directory containing source files for a monster.")
        .index(1)
        .required(true),
    )
    .arg(
      Arg::with_name("dest")
        .help("Path to save imported Nova assets to.")
        .index(2)
        .required(true),
    );

  // Get all matching arguments from the command line.
  let matches = app.get_matches();

  let mut src_path = PathBuf::from(matches.value_of_os("src").unwrap());
  let mut dest_path = PathBuf::from(matches.value_of_os("dest").unwrap());

  // Ensure dest path exists.
  fs::create_dir_all(&dest_path)?;

  // Copy the monster's sprite sheet.
  src_path.push("sheet.png");
  dest_path.push("atlas.png");

  fs::copy(&src_path, &dest_path)?;

  src_path.pop();

  // Load the animations.xml data.
  src_path.push("animations.xml");

  let anim_data = animations::load(&src_path)?;

  src_path.pop();

  // Save the sprite atlas metadata.
  dest_path.set_extension("yml");

  core::fs::save_yaml(
    &dest_path,
    &graphics::sprite::atlas::Data {
      cell_width: anim_data.frame_width,
      cell_height: anim_data.frame_height,
    },
  )?;

  dest_path.pop();

  // Convert animations to sequences.
  let mut sequences = Vec::new();

  build_sequences(
    "walk",
    &anim_data,
    animations::Type::Walk as usize,
    &mut sequences,
  );

  // Save sequences.yml.
  dest_path.push("sequences.yml");

  core::fs::save_yaml(&dest_path, &sequences)?;

  Ok(())
}

/// Builds sprite atlas animations from audino animation data.
fn build_sequences(
  name: &str,
  input: &animations::AnimData,
  group_index: usize,
  output: &mut Vec<graphics::sprite::animation::Sequence>,
) {
  for (i, sequence_index) in input.group_table.groups[group_index]
    .sequence_indices
    .iter()
    .enumerate()
  {
    let sequence = &input.sequence_table.sequences[*sequence_index];

    output.push(graphics::sprite::animation::Sequence {
      name: format!("{}_{}", name, animations::DIRECTONS[i]),
      frames: sequence
        .frames
        .iter()
        .map(|f| graphics::sprite::animation::Frame {
          cell: (f.meta_frame_group_index % 8, f.meta_frame_group_index / 8),
          length: f.duration as f64,
          hflip: f.hflip != 0,
        })
        .collect(),
    });
  }
}
