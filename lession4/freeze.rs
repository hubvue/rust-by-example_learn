// 可变变量与不可变变量可以相互遮蔽

fn main() {
  let mut _mutable_integer = 7i32;
  {
    let _mutable_integer = _mutable_integer;
    // _mutable_integer = 10;
    println!("{}", _mutable_integer);
  }
  _mutable_integer = 10;
  println!("{}", _mutable_integer);
  let _test = 7i32;
  {
    let mut _test = _test;
    println!("test: {}", _test);
    _test = 100;
    println!("test: {}", _test);
  }
  // _test = 200;
  println!("test: {}", _test);
}
