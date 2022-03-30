// ToString
// 要把任何类型转为String，只需要实现哪个类型的ToString trait, 然而不要直接这么做，可以实现fmt::Display trait，它会自动提供ToString，并且还可以用来打印类型。

use std::string::ToString;
use std::fmt;

struct Circle {
  radius: i32
}

// 实现ToString trait
impl ToString for Circle {
  fn to_string(&self) -> String {
    format!("Circle of radius {:?}", self.radius)
  }
}

struct Circle1 {
  radius: i32
}

// 实现Display
impl fmt::Display for Circle1 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Circle1 of radius {}", self.radius)
  }
}

fn main() {
  let circle = Circle {
    radius: 6
  };
  let circle1 = Circle1 {
    radius: 8
  };
  println!("{}", circle.to_string());
  println!("{}", circle1);
}
