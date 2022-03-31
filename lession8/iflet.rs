

fn main() {
  // 在一些场合下， 用match匹配枚举类型并不优雅，例如：
  let optional = Some(7);
  match optional {
    Some(i) => println!("Hello {}", i),
    _ => ()  // 必须要有，因为match需要覆盖全部情况，不觉得多余吗？
  }

  // if let 在这样的场合就简洁的多，并且允许指明数种失败情形下的选项：
  let number = Some(7);
  let letter: Option<i32> = Some(8);
  let emotion: Option<i32> = None;

  // if let 结构读作：若 let 将 number 解构成 Some(i) 则执行代码块中的代码。
  if let Some(i) = number {
    println!("Matched {:?}", i);
  }

  // 如果要指明失败的情形，就使用else：
  if let Some(i) = letter {
    println!("Matched {:?}!", i);
  } else {
    // 解构失败
    println!("Did not match a number.");
  }

  // 提供另外一种失败的形式
  let _i_like_letters = false;
  if let Some(i @ 7) = emotion {
    println!("Matched {:?}!", i);
  } else if let Some(i @ 8) = letter {
    println!("Did not match a number {:?}.", i);
  } else {
    println!("I do not like letters");
  }

  // 也可以用if let 匹配任何枚举值
  #[derive(Debug)]
  enum Foo {
    Bar,
    Baz,
    Qux(u32)
  }

  let a = Foo::Bar;
  let b = Foo::Baz;
  let c = Foo::Qux(100);

  if let Foo::Bar = a {
    println!("a  is foobar");
  }
  if let Foo::Bar = b {
    println!("b is foobar");
  }

  if let Foo::Qux(value) = c {
    println!("c is {}", value);
  }
  println!("{:?}, {:?}, {:?}", a, b, c);

  // if let 允许匹配枚举非参数化的变量，即枚举未注明#[derive(PartialEq)],
  // 我们也没有为其实现PartialEa。
  // 通常 if Foo::Bar == a 会出错，因为此类枚举的实例不具有可比性。但是if let 是可行的。
  // #[derive(PartialEq)]
  enum Foo1 {
    Bar
  }
  let a = Foo1::Bar;
  // if Foo1::Bar == a { // error，只有加上PartialEq，才ok
  //   println!("a is foobar");
  // }

  if let Foo1::Bar = a {
    println!("a is foobar");
  }
}
