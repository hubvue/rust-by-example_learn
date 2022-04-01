// 用函数方式重写fizzBuzz程序

fn main() {
  fizzbuzz_to(100);
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
  if rhs == 0 {
    return false;
  }
  lhs % rhs == 0  // 表达式可以不用return关键字
}

// 一个没有返回值的函数，实际上会返回一个单元类型 ()
fn fizzbuzz(n: u32) -> () {
  if is_divisible_by(n, 15) {
    println!("fizzbuzz");
  } else if is_divisible_by(n, 3){
    println!("fizz");
  } else if is_divisible_by(n, 5) {
    println!("buzz");
  } else {
    println!("{}", n);
  }

  match n {
    v if is_divisible_by(v, 15) => println!("fizzbuzz"),
    v if is_divisible_by(v, 3) => println!("fizz"),
    v if is_divisible_by(v, 5) => println!("buzz"),
    v => println!("{}", v)
  }

  // 那种更简洁，我反而倾向于第二种
}

fn fizzbuzz_to(n: u32) {
  for n in 1..=n {
    fizzbuzz(n); // 当函数返回 () 时，函数签名可以省略范湖类型
  }
}
