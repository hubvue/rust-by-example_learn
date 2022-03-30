// if-else 条件选择可以是一个表达式，并且所有分支必须返回相同的类型


fn main() {
  let n = 5;

  if n < 0 {
    println!("{} is negative", n);
  } else if n > 0 {
    println!("{} is positive", n);
  } else {
    print!("{} is zero", n);
  }

  let big_n = 
        if n < 10 && n > -10 {
          println!(", and is a small number, increase ten-fold");
          // 这个表达式返回一个i32类型
          10 * n
        } else {
          println!(", and is a big number, half the number");
          // 这个表达式也必须返回一个i32类型
          n / 2
        };
  
  println!("{} -> {}", n, big_n);
    
}
