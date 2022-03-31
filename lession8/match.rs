// Rust 通过match关键字来提供模式匹配，和switch用法类似。
// 第一个匹配分支会被比对，并且所有可能的分支都必须被覆盖

#[allow(dead_code)]
enum Color {
  Red,
  Blue,
  Green,
  RGB(u32, u32, u32),
  HSV(u32, u32, u32),
  HSL(u32, u32, u32),
  CMY(u32, u32, u32),
  CMYK(u32, u32, u32, u32)
}

fn main() {
  for n in 1..20 {
    match n {
      // 单值匹配
      1 => println!("One!"),
      // 多值匹配
      2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
      // 匹配一个区间
      13..=19 => println!("A teen"),
      _ => println!("Ai not spectial")
    }
  }

  let boolean = true;
  let binary = match boolean {
    // match 分支必须覆盖所有可能的值
    false => 0,
    true => 1
  };
  println!("{}, {}", boolean, binary);

  // match 解构
  // 解构元组
  println!("-------------解构元组---------------");
  let triple = (2, -2, 3);
  println!("Tell me about {:?}", triple);
  match triple {
    // 解构元组元素
    (0, y, z) => println!("x: 0, y: {}, z: {}", y, z),
    // .. 可以用来忽略元组的其余部分
    (1, ..) => println!("x: 123"),
    // _ 表示不将值绑定到变量
    _ => println!("It does not matter what they are")
  }

  // 解构枚举
  println!("-------------解构枚举---------------");

  let color = Color::RGB(127, 17, 40);

  println!("What color is it?");

  match color {
    Color::Red => println!("Red"),
    Color::Blue => println!("Blue"),
    Color::Green => println!("Green"),
    Color::RGB(r, g, b) => println!("r: {}, g: {}, b: {}", r, g, b),
    Color::HSV(h, s, v) => println!("h: {}, s: {}, v: {}", h, s, v),
    Color::HSL(h, s, l) => println!("h: {}, s: {}, l: {}", h, s, l),
    Color::CMY(c, m, y) => println!("c: {}, m: {}, y: {}", c, m, y),
    Color::CMYK(c, m, y, k) => println!("c: {}, m: {}, y: {}, k: {}", c, m, y, k)
  }

}
