// for in 结构可以遍历一个Iterator.
// 创建迭代器的一个最简单的方法是使用区间标记 a..b  [a, b)
// 或者可以使用 a..=b 即 [a, b]

fn main() {
  for n in 1..101 {
    if n % 15 == 0 {
      println!("fizzbuzz");
    } else if n % 3 == 0 {
      println!("fizz");
    } else if n % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", n);
    }
  }

  for n in 1..=100 {
    if n % 15 == 0 {
      println!("fizzbuzz");
    } else if n % 3 == 0 {
      println!("fizz");
    } else if n % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", n);
    }
  }
}
