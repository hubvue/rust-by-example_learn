// 既然闭包可以作为参数，闭包是一个函数，那么函数也可以作为其他函数的参数。
// 类似于高阶函数。
// 如果声明一个接受闭包作为参数的函数，那么任何满足该闭包的trait约束的函数都可以作为其参数。


// 定义一个函数，可以接受一个由Fn限定的泛型F参数并调用它
// fn call_me<F: Fn()>(f: F) {
//   f()
// }

fn call_me<F>(f: F) where F: Fn() {
  f()
}

// 定义一个满足Fn约束的封装函数
fn function() {
  println!("I am a function!")
}

fn main() {
  // 定义一个满足Fn约束的闭包
  let closure = || println!("I am a closure!");

  call_me(closure);
  call_me(function);
}
