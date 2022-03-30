// Rust 提供了 loop 关键字来表示一个无限循环。
// 可以使用break语句在任何时候退出一个循环，还可以使用continue跳过循环体的圣墟部分并开始下一轮循环。


fn main() {
  let mut count = 0u32;

  loop {
    count += 1;
    if count == 3 {
      println!("three");
    }
    println!("{}", count);

    if count == 5 {
      println!("Ok, that is enough");
      break;
    }
  }
}
