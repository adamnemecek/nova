use nova::*;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
  let cli = clap::App::new("tvb").arg(
    clap::Arg::with_name("vfs-mount")
      .long("vfs-mount")
      .short("v")
      .number_of_values(2)
      .value_names(&["VFS_PATH", "REAL_PATH"])
      .multiple(true)
      .help("Adds a virtual file system mount at <VFS_PATH> pointing to the real file system at <REAL_PATH>."),
  );

  let matches = cli.get_matches();

  let mut vfs = vfs::Context::new();

  if let Some(mut mounts) = matches.values_of("vfs-mount") {
    while let (Some(prefix), Some(fs_path)) = (mounts.next(), mounts.next()) {
      vfs.mount(prefix, fs_path);
    }
  }

  use std::io::{Read, Write};

  let mut file = vfs.create("/saves/lol")?;

  file.write_fmt(format_args!("hello world"))?;

  file = vfs.open("/control_map.toml")?;

  let mut string = String::new();

  file.read_to_string(&mut string)?;

  print!("{}", &string);

  Ok(())

  /*
  // Create a terminal logger and set it as the global default.
  let logger = log::terminal_compact();

  log::set_global_logger(&logger);

  // Create a graphics context and background loader.
  let graphics = gfx::Context::new(&logger)?;
  let loader = gfx::Loader::new(&graphics)?;

  // Open a window.
  let window = window::open(window::Options {
    size: (2560.0, 1440.0).into(),
    resizable: false,
    ..Default::default()
  });

  // Start the renderer.
  let renderer = gfx::render::start(&graphics, &window, &loader, &logger)?;

  // Run the main game loop 60 times per second.
  time::loop_at_frequency(60.0, |main_loop| {
    while let Some(event) = window.next_event() {
      match event {
        // When the user tries to close the window, exit the game loop.
        window::Event::CloseRequested => {
          log::info!(logger, "close requested");

          return main_loop.stop();
        }

        // When the window is resized, resize the render surface as well.
        window::Event::Resized => {
          renderer.resize_surface(window.size());
        }
      }
    }
  });

  // Shut down the renderer before exiting to clean up resources.
  renderer.shut_down();
  */
}
