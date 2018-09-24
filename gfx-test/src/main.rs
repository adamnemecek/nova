#[cfg(windows)]
use gfx_backend_dx12 as backend;
#[cfg(target_os = "macos")]
use gfx_backend_metal as backend;
#[cfg(all(unix, not(target_os = "macos")))]
use gfx_backend_vulkan as backend;
use gfx_hal as hal;
use winit;

const WINDOW_TITLE: &str = "gfx-test";

pub fn main() {
  // Create the window and events loop.

  let mut events_loop = winit::EventsLoop::new();

  let window = winit::WindowBuilder::new()
    .with_title(WINDOW_TITLE)
    .build(&events_loop)
    .expect("could not build window");

  // Run the events loop until the window is closed.
  events_loop.run_forever(|event| {
    match event {
      winit::Event::WindowEvent { ref event, .. } => match event {
        winit::WindowEvent::CloseRequested => return winit::ControlFlow::Break,
        _ => (),
      },
      _ => (),
    }

    println!("{:?}", event);

    winit::ControlFlow::Continue
  });
}
