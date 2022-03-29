// 对于数值字面量，主要把类型作为后缀加上去，就完成了类型说明。
// 比如字面量42的类型时i32 即 42i32

// 无后缀的数值字面量，其类型取决于怎样使用它们。如果没有限制，编译器会对整数使用i32，对浮点数使用f64

fn main() {
  // 带后缀的字面量
  let x = 1u8;
  let y = 2u32;
  let z = 3f32;
  println!("x: {}, y: {}, z: {}",x, y, z);

  // 无后缀的字面量，其类型取决于如何使用它们
  let i = 1;
  let f = 1.2;

  // size_of_val：返回一个变量所占的字节数
  println!("x: {}", std::mem::size_of_val(&x));
  println!("y: {}", std::mem::size_of_val(&y));
  println!("z: {}", std::mem::size_of_val(&z));
  println!("i: {}", std::mem::size_of_val(&i));
  println!("f: {}", std::mem::size_of_val(&f));
}
