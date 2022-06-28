struct Color(u8, u8, u8);
struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    }
  }

  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }

  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
    // (String::from("hoge"), String::from("fuga"))
  }
}

pub fn run() {
  let mut c = Color(255, 0, 0);
  c.0 = 200;
  println!("Color: {} {} {}", c.0, c.1, c.2);

  let mut p = Person::new("Niran", "Ken");
  println!("Person {} {}", p.first_name, p.last_name);
  p.set_last_name("fuga");
  println!("Person {}", p.full_name());
  println!("Person Tuple {:?}", p.to_tuple());
}
