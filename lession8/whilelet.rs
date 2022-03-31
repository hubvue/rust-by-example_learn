
fn main() {
  // 和 if let 类似， while let 也可以把 别扭的 match 改写的好看一些。
  let mut optional = Some(0);

  loop {
    match optional {
      Some(i) => {
        if i > 9 {
          println!("Greater than 9, quit!");
          optional = None;
        } else {
          println!("i is {:?}", i);
          optional = Some(i + 1);
        }
      },
      _ => { break; }
    }
  }

  // 使用while let 改写上面代码
  let mut optional = Some(0);

  while let Some(i) = optional {
    if i > 9 {
      println!("Greater than 9, quit!");
      optional = None;
    } else {
      println!("i is {:?}", i);
      optional = Some(i + 1);
    }
  }
}
