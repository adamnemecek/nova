mod config;

use nova::*;
use std::{io, process};

pub fn main() {
  // Initialize logging.
  log::init();

  let result = run();

  // Shut down logging to flush pending messages.
  log::shut_down();

  process::exit(result);
}

fn run() -> i32 {
  let log = log::default();

  // Parse CLI args with clap.
  let cli_args = clap::App::new("tvb").arg(
    clap::Arg::with_name("vfs-mount")
      .long("vfs-mount")
      .number_of_values(2)
      .value_names(&["VFS_PATH", "REAL_PATH"])
      .multiple(true)
      .help("Adds a virtual file system mount at <VFS_PATH> pointing to the real file system at <REAL_PATH>."),
  ).get_matches();

  // Create a new virtual file system and mount paths according to the
  // command-line arguments.
  let vfs = vfs::new();

  if let Some(mut mounts) = cli_args.values_of("vfs-mount") {
    while let (Some(prefix), Some(fs_path)) = (mounts.next(), mounts.next()) {
      vfs.mount(prefix, fs_path);
    }
  }

  // Load configuration from the virtual file system.
  let config = match vfs.read_to_string("/tvb.toml") {
    Ok(toml) => match config::from_toml(&toml) {
      Ok(config) => config,

      Err(err) => {
        log::crit!(&log, "could not parse tvb.toml: {}", err);
        return 1;
      }
    },

    Err(err) => {
      if err.kind() == io::ErrorKind::NotFound {
        log::crit!(&log, "could not find tvb.toml");
      } else {
        log::crit!(&log, "could not read tvb.toml: {}", err);
      }

      return 1;
    }
  };

  // Open a window as configured.
  let window = window::open(window::Options {
    title: "tvb".into(),
    size: config.window.size(),
    resizable: config.window.resizable,
  });

  // Process window events every 60th of a second.
  time::loop_at_frequency(60.0, |main_loop| {
    while let Some(event) = window.next_event() {
      if let window::Event::CloseRequested = event {
        log::info!(&log, "close requested");

        return main_loop.stop();
      }
    }
  });;

  0
}
