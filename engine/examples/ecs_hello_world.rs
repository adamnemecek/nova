// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova::ecs;

fn main() {
  // Create a new ECS context. This represents the state of all ECS resources,
  // including entities and components.
  let mut ctx = ecs::Context::new();

  // Create a new dispatcher. A dispatcher creates an execution plan so that
  // systems that do not mutate the same resources can run simultaneously on
  // separate threads.
  let mut dispatcher = ecs::Dispatcher::new()
    // Add a `Greeter` as a new system named `"Greeter"` with no dependencies.
    .system("Greeter", &[], Greeter)
    // Set up the dispatcher and all of its systems. Calls `Greeter::setup()`.
    .setup(&mut ctx);

  // Dispatches all systems once. In this case, it calls `Greeter::run` which
  // prints `"Hello world."`, the default message.
  dispatcher.dispatch(&mut ctx);

  // Gets a mutable reference to the `Greeting` resource to change its message.
  let greeting = ecs::get_resource_mut::<Greeting>(&mut ctx);

  greeting.message = "Hallo Welt!".into();

  // This time, `Greeter::run` will print `"Hallo Welt!"`.
  dispatcher.dispatch(&mut ctx);
}

// A resource read by `Greeter` to determine what to print on the screen.
struct Greeting {
  message: String,
}

// A system that prints a message on the screen.
struct Greeter;

impl<'a> ecs::System<'a> for Greeter {
  type Data = ecs::ReadResource<'a, Greeting>;

  fn setup(&mut self, ctx: &mut ecs::Context) {
    // Add a `Greeting` resource with a default message.
    ecs::put_resource(
      ctx,
      Greeting {
        message: "Hello world.".into(),
      },
    );
  }

  fn run(&mut self, greeting: Self::Data) {
    println!("{}", greeting.message);
  }
}
