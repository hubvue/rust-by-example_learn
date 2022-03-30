// 解析字符串
// 我们经常需要把字符串转为数字。完成这项工作的标准手段是用parse函数。我们得提供奥转换到的类型，这样可以通过不适用类型推断。

// 只要对目标类型实现了FromStr trait，就可以用parse把字符串转为目标类型。
// 标准库中已经给无数中类型实现了FromStr如果要转换到用户定义类型只要手动实现FromStr就行。


use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Number {
  value: i32
}

impl FromStr for Number {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Number{
      value: s.parse().unwrap()
    })
  }
}
fn main() {
  let parsed: i32 = "5".parse().unwrap();
  let turbo_parsed = "10".parse::<i32>().unwrap();
  let sum = parsed + turbo_parsed;
  println!("sum: {:?}", sum);

  let num: Number = "3".parse().unwrap();
  println!("{:?}", num);
  assert_eq!(num, Number{value: 3});
}
