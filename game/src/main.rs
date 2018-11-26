// TODO: Remove when RLS supports it.
extern crate nova;

use nova::window::{self, Window};

pub fn main() {
  let mut engine = nova::Engine::new();

  engine.put_resource(window::Settings {
    fullscreen: true,
    ..Default::default()
  });

  let mut window = Window::create(&mut engine).expect("Could not create window");

  window.update(&mut engine);

  std::thread::sleep(std::time::Duration::from_secs(1));

  let settings: &mut window::Settings = engine.get_resource_mut();

  settings.fullscreen = false;

  window.update(&mut engine);

  loop {
    window.update(&mut engine);

    std::thread::sleep(std::time::Duration::from_millis(1));
  }
}
