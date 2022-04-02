// Rust中的闭包也叫做lambda表达式或者lambda，是一类能够捕获周围作用域中变量的函数。
// 例如，一个可以捕获 x 变量的闭包如下：
//  |val| val + x

// 它们的语法和能力使它们在临时使用时非常方便。调用一个闭包和调用一个函数完全相同，㘝调用闭包时，输入和返回类型二者都可以自动推导，而输入变量名必须指明。

// 其他的特点包括：
// 1.声明时使用 || 代替 () 将输入参数括起来
// 2.函数体定界符({})对于单个表达式是可选的，其他情况必须加上。
// 3.有能力捕获外部环境的变量。

fn main() {
  // 通过闭包和函数分别实现自增
  fn function(i: i32) -> i32 {i + 1}

  // 闭包时匿名的，可以绑定到变量
  // 类型标注和函数一样
  let closure_annotated = |i: i32| -> i32 { i + 1 };
  let closure_inferred = |i| i + 1;

  let i = 1;
  println!("function: {}", function(i));
  println!("closure_annotated: {}", closure_annotated(i));
  println!("closure_inferred: {}", closure_inferred(i));

  // 没有参数的闭包，返回一个i32的类型。
  // 返回类型时自动推导的。
  let one = || 1;
  println!("closure returning one: {}", one());
}
