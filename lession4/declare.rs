// 变量可以先声明，后初始化，但是必须在使用前初始化

fn main() {
  let a_binding;
  {
    let x = 2;
    a_binding = x * x;
  }

  println!("{}", a_binding);

  let another_binding;
  // 初始化前使用，报错
  // println!("{}", another_binding);

  another_binding = 1;
  println!("{}", another_binding);
}
