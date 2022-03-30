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

  // 嵌套循环
  'outer: loop {
    println!("Entered the outer loop");

    'inner: loop {
      // 这只是中断内部的循环
      // break;

      // 通过标签break
      break 'outer;
    }

    println!("This point will never be reached");
  }
  println!("Exited the outer loop");

  // loop 返回值
  let mut counter = 0;
  let result = loop {
    counter += 1;
    if counter == 10 {
      break counter * 2;
    }
  };
  assert_eq!(result, 20);
}
