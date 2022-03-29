// Rust中有两种常量，可以在任意作用域中声明。他们都需要显式的类型声明：
// - const: 不可改变的值
// - static: 具有 static 生命周期


// 全局变量是在所有其他作用域之外声明的。
static LANGUAGE: &'static str = "Rust";
const THRESHOLD:i32 = 10;

fn is_big(n: i32) -> bool {
  n > THRESHOLD
}

fn main() {
  let n = 16;
  println!("This is {}", LANGUAGE);
  println!("This threshold is {}", THRESHOLD);
  println!("{} is {}", n, if is_big(n) { "Big"} else {"small"});

  // 不能修改一个常量
  // THRESHOLD = 5;
}
