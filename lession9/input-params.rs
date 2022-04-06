// 虽然Rust无需类型说明就能在大多数时候完成变量捕获，但在编写函数时，这种模糊的写法是不允许的。
// 当以闭包作为输入参数时，必须指出闭包的完整类型，它是通过使用以下trait中的一种来指定的。
// 其受限制程度按以下顺序递减。
// - Fn: 表示捕获方式通过引用(&T)的闭包
// - FnMut: 表示捕获方式为通过可变引用(&mut T)的闭包
// - FnOnce: 表示捕获方式为通过值(T)的闭包
// 顺序之所以是这样是因为 &T 只是说获取了不可变的引用；&mut T则可以改变变量；T 则是拿到了变量的所有权而非借用。

// 对闭包索要捕获的每个变量，事实上编译器在满足使用需求的前提下尽量以限制最多的方式捕获。
// 例如用一个类型说明为FnOnce的闭包作为参数。这说明闭包可能采取&T, &mut T 或 T中的一种捕获方式。
// 但编译器最终是根据所捕获变量在闭包里的使用情况决定捕获方式。

// 这是因为如果能以移动的方式捕获变量，则闭包也有能力使用其他方式借用变量。
// 反过来则不行：如果参数的类型说明为Fn，那么不允许闭包通过&mut T 或 T 捕获变量。

// fn apply<F>(f: F) where F: FnOnce() { // 闭包没有输入值和返回值
//   f();
// }

fn apply<F: FnOnce()>(f: F) { // 闭包没有输入值和返回值
  f();
}

// 闭包处理一个i32，并返回一个i32
// fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
//   f(3)
// }

fn apply_to_3<F: Fn(i32) -> i32>(f: F) -> i32 {
  f(3)
}

fn main() {
  use std::mem;

  let greeting = "Hello";
  let mut fatewell = "goodbye".to_owned();

  let diary = || {
    // greeting 通过引用捕获，顾需要闭包时Fn
    println!("I said {}", greeting);

    // 改变了fatewell，因此要求闭包使用可变引用来捕获。 FnMut
    fatewell.push_str("!!!");
    println!("Then I screamed {}", fatewell);
    println!("Now I can sleep. zzzz");

    // 手动调用drop，要求闭包通过值来捕获fatewell FnOnce
    mem::drop(fatewell);
  };

  apply(diary);

  let double = |x| 2 * x;
  println!("{}", apply_to_3(double));
}
