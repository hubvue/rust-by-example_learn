// 闭包也可以作为一个函数的返回值。
// 然而返回闭包类型会有问题，因为目前Rust只支持返回具体的（非泛型）类型。
// 按照定义，匿名的闭包的类型时未知的，所以只有使用impl Trait才能返回一个闭包。

// 返回闭包的有效特征有：
// - Fn
// = FnMut
// - FnOnce

// 除此之外，还必须使用move关键字，它表明所有的捕获都是通过值进行的。这是必须得，因为在函数退出时，任何通过引用的捕获都会被丢弃(这一点和js不同)，在闭包中留下无效的引用。

fn create_fn() -> impl Fn() {
  let testt = "Fn".to_owned();

  move || println!("This is a : {}", testt)
}

fn create_fnmut() -> impl FnMut() {
  let text2 = "FnMut".to_owned();

  move || println!("This is a: {}", text2)
}

fn create_fnonce() -> impl FnOnce() {
  let text1 = "FnOnce".to_owned();

  move || println!("This is a: {}", text1)
}


fn main() {
  let fn_plain = create_fn();
  let mut fn_mut = create_fnmut();
  let fn_once = create_fnonce();

  fn_plain();
  fn_mut();
  fn_once();
}
