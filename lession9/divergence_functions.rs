// 发散函数
// 发散函数绝不会返回。它们使用！标记，这是一个空类型。


// #![feature(never_type)]


// fn foo() -> ! {
//   panic!("This call never returns.");
// }

// 和所有其他类型相反，这个类型无法实例化，因为此类型可能具有所有可能的值的集合为空。
// 注意：它和()类型不同，后者只有一个可能的值。

// fn some_fn() {
//   ()
// }


// fn main() {
//   let a: () = some_fn();
//   println!("This function returns and you can see this line. {:?}", a);

//   // 下面这个函数相反，这个函数永远不会将控制内容返回给调用者。

//   let x: ! = panic!("This call never returns.");
//   println!("You will never  see this line!");
// }


// 虽然这看起来像是一个抽象的概念，但实际上这非常有用且方便。这种类型的主要优点是它可以被转换为任何其他类型，
// 从而可以在需要精确类型的地方使用，例如在match匹配分支。这允许我们编写如下代码。

fn main() {
  fn sum_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;
    for i in 0..up_to {
      // 注意这个match表达式的返回值必须为u32
      // 因为addition 变量是这个类型。
      let addition: u32 = match i % 2 == 1 {
        // i 变量的类型为 u32，这毫无问题
        true => i,
        // 另一方面，continue表达式不返回u32，但它仍然没有问题。
        // 因为它永远不会返回，因此不会违反匹配表达式的类型要求。
        false => continue
      };
      acc += addition;
    }
    acc
  }
  println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}

// 这也是永远循环函数（如 loop{} 以及 网络服务器）或终止进程的函数 exit() 的返回类型。
