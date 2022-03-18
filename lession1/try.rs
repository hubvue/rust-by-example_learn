// 对一个结构体实现fmt::Display，其中的元素需要一个接一个的处理到，这可能会很麻烦。问题在于每个write!都要生成一个fmt::Result。正确的实现需要处理所有的Result。
// Rust专门为解决这个问题提供了 `?`操作符。

// ？表示对`write!` 进行尝试(try) 观察是否出错。若发生错误，返回相应的错误；否则(没有出错)继续执行后面的语句。
// write!(f, "{}", value)?;
// 等价于
// try!(write!(f, "{}", value));
// ? 实际上就是try宏

use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let vec = &self.0;

    write!(f, "[")?;

    // 使用 v 对 vec 进行迭代，并用count记录迭代次数。
    for (count, v) in vec.iter().enumerate() {
      if count != 0 {
        write!(f, ", ")?;
      }
      write!(f, "{}: {}", count, v)?;
    }

    write!(f, "]")
  }
}


fn main() {
  let v = List(vec![1,2,3]);
  println!("{}", v);
}
