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

  // 解构指针和引用
  println!("-------------解构指针和引用---------------");
  // 对指针来说，解构和解引用要区分，因为这二者的概念是不同的。
  // 解引用使用*
  // 解构使用 &、ref和ref mut

  let reference = &4; // 获得一个i32类型的引用
  match reference {
    // 如果用 &val 这个模式去匹配 reference，就相当于这样的比较：
    // &i32 即 reference 的类型
    // &val 即 用于匹配的模式
    // 进行匹配，如果去掉匹配的&， 那么 val 就是i32，
    &val => println!("Got a value via : {:?}", val)
  }
  // 如果不想用 & 来结解构引用， 需要在匹配前来解引用
  match *reference {
    val => println!("Got a value via: {:?}", val)
  }

  // 如果一开始就不是引用
  let _not_a_reference = 3;
  // Rust 对这种情况提供了 ref。它更改了赋值行为，从而可以对具体值创建引用
  // 下面代码得到一个引用
  let ref _is_a_reference = 3;
  match _is_a_reference {
    ref r => println!("value: {}", r)
  }

  // 相应的，定义两个非引用的变量，通过 ref 和 ref mut应该可以其到其引用
  let value = 5;
  let mut mut_value = 6;

  match value {
    ref r => println!("value: {}", r)
  }
  match mut_value {
    ref mut m => {
      *m += 10;
      println!("value: {:?}", m);
    }
  }
}
