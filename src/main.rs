mod engine;
mod systems;

use self::engine::Engine;
use nova::specs::ReadExpect;
use std::error::Error;


pub fn main() -> Result<(), Box<dyn Error>> {
  let engine = Engine::new();

  engine.execute(|engine| {
    engine.put_resource(TestRes {
      message: String::from("world"),
    });

    engine.add_system(TestSystem);
  });

  engine.dispatch(TestMessage {
    content: String::from("Hello"),
  });

  std::thread::sleep(std::time::Duration::from_secs(1));

  Ok(())
}

struct TestSystem;

impl<'a> systems::System<'a, TestMessage> for TestSystem {
  type Data = ReadExpect<'a, TestRes>;

  fn run(&mut self, msg: &TestMessage, data: Self::Data) {
    println!("{} {}", &msg.content, &data.message);
  }
}

struct TestMessage {
  content: String,
}

struct TestRes {
  message: String,
}
