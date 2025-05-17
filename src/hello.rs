pub struct HelloWorld {
  name: String,
}

impl HelloWorld {
  pub fn new(user: &str) -> Self {
    HelloWorld {
      name: user.to_string(),
    }
  }

  pub fn hello(&self) -> String {
    format!("Hello {} from Rust library!", self.name)
  }

  pub fn say_hello(&self) {
    let message = self.hello();
    println!("{}", message);
  }
}
