// 变量绑定默认是不可变的，但是加上mut修饰后变量就可以改变。

fn main() {
  let _immutable_binding = 1;
  let mut mutable_binding = 1;
  println!("Before: {}", mutable_binding);

  mutable_binding += 1;
  println!("After: {}", mutable_binding);
}
