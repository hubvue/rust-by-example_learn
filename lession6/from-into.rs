// From Into
// From和Into两个trait是内部相关联的。如果我们能够从类型B得到类型A，那么我们也可以从类型A得到类型B

// From
// From trait 允许一种类型定义”怎么根据另一种类型生成自己“（感觉类似构造函数），因此它提供了一种类型转换的简单机制。
// 在标准库中有无数From的实现，规定原生类型及其他常见类型的转换功能。

// Into
// Into trait 就是把From trait 倒过来而已，也就是说：如果类型实现了From，那么同时也就免费获得了Into。
// 使用了Into trait 通常要求指明转换到的类型，因为编译器大多数情况下不能推断它。

use std::convert::From;

#[derive(Debug)]
struct Number {
  value: i32
}

impl From<i32> for Number {
  fn from(item: i32) -> Self {
    Number{
      value: item
    }
  }
}

fn main() {
  // 将 str类型转换为String类型
  let my_str = "Hello world";
  let my_string = String::from(my_str);
  println!("{:?}", my_string);

  // 自定义类型转换机制
  let num = Number::from(30);
  println!("{:?}, {}", num, num.value);

  let int = 5i32;
  let num: Number = int.into();
  println!("My number is {:?}", num);
}
