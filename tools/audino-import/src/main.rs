// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate clap;
extern crate nova_engine;

use clap::{App, Arg};
use nova_engine::sprites::AtlasData;
use std::fs;
use std::path::PathBuf;

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
    )
    .arg(
      Arg::with_name("columns")
        .short("c")
        .value_name("NUM")
        .help("Number of columns in the sprite sheet.")
        .default_value("8")
        .required(true),
    )
    .arg(
      Arg::with_name("rows")
        .short("r")
        .value_name("NUM")
        .help("Number of rows in the sprite sheet.")
        .required(true),
    );

  // Get all matching arguments from the command line.
  let matches = app.get_matches();

  let mut src_path = PathBuf::from(matches.value_of_os("src").unwrap());
  let mut dest_path = PathBuf::from(matches.value_of_os("dest").unwrap());

  let columns = matches
    .value_of("columns")
    .unwrap()
    .parse()
    .expect("invalid columns");

  let rows = matches
    .value_of("rows")
    .unwrap()
    .parse()
    .expect("invalid rows");

  fs::create_dir_all(&dest_path)?;

  // Copy the monster's sprite sheet.
  src_path.push("sheet.png");
  dest_path.push("atlas.png");

  fs::copy(&src_path, &dest_path)?;

  // Save the sprite atlas metadata.
  dest_path.set_extension("yml");

  AtlasData { columns, rows }.save(&dest_path)?;

  Ok(())
}
