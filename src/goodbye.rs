pub struct GoodbyeWorld {
  name: String,
}

impl GoodbyeWorld {
  pub fn new(user: &str) -> Self {
    GoodbyeWorld {
      name: user.to_string(),
    }
  }

  pub fn goodbye(&self) -> String {
    format!("Goodbye {} from Rust library!", self.name)
  }

  pub fn say_goodbye(&self) {
    let message = self.goodbye();
    println!("{}", message);
  }
}
