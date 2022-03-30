// while 关键字可以用作当型循环

fn main() {
  let mut n = 1;

  while n < 101  {
    if n % 15 == 0 {
      println!("fizzbuzz");
    } else if n % 3 == 0 {
      println!("fizz");
    } else {
      println!("{}", n)
    }
    // rust里没有n ++
    n += 1;
  }
}
