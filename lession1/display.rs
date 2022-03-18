// fmt::Debug 通常开起来不太简洁，因此自定义输出的外观经常更可取。这需要通过手动实现fmt::Display来做到。fmt::Display采用{}标记。

/*

// 导入fmt模块
use std::fmt;

// 定义一个结构体，
struct Structure(i32);

// 手动实现Display
impl fmt::Display for Structure {
  // 这个trait 要求 fmt 使用与下面的函数完全一直的函数签名。
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // 仅将 self 的第一个元素写入给定的输出流 f。返回 fmt.Result,
    // 此结果表明操作成功或失败
    write!(f, "{}", self.0)
  }
}


fn main() {
  println!("{}", Structure(100));
}
*/

/*

use std::fmt;

// 带有两个数字的结构体，推导出Debug和实现Display输出进行比较
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.0, self.1)
  }
}

// 定义一个含有具名字段的结构体
#[derive(Debug)]
struct Point2D {
  x: f64,
  y: f64,
}

impl fmt::Display for Point2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "x: {}, y: {}", self.x, self.y)
  }
}

fn main() {
  let minmax = MinMax(0, 14);
  println!("Compare structures:");
  println!("Display: {}", minmax);
  println!("Debug: {:?}", minmax);

  let big_range = MinMax(-300, 300);
  let small_range = MinMax(-3, 3);

  println!("The big range is {big} and the small is {small}", small = small_range, big = big_range);

  let point = Point2D{
    x: 3.3,
    y: 7.2
  };

  println!("Compare points:");
  println!("Dsiaplay: {}", point);
  println!("Debug: {:?}", point);
}

*/

use std::fmt;

#[derive(Debug)]
struct Complex {
  real: f64,
  imag: f64
}

impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}+{}i", self.real, self.imag)
  }
}

fn main() {

  let complex = Complex {
    real: 3.3,
    imag: 7.2
  };
  println!("Display: {}", complex);
  println!("Debug: {:?}", complex);
}
