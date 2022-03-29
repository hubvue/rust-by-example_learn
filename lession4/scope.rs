// 变量绑定有一个作用域，它被限定只在一个代码块中生存。代码块是一个被{}包围的语句集合。另外也允许变量遮蔽

fn main() {
  // 此绑定生存于main函数中
  let long_lived_binding = 1;
  // 这是一个代码块，比 main 函数拥有更小的作用域
  {
    // 此绑定只存在于本代码块
    let short_lived_binding = 2;

    println!("short_lived_binding: {}", short_lived_binding);

    // 此绑定遮蔽了外面的绑定
    let long_lived_binding= 5_f32;
    println!("long_lived_binding: {}", long_lived_binding);
  }

  // 访问一个当前作用域未定义的变量时报错
  // println!("{}", short_lived_binding)

  println!("{}", long_lived_binding);
  // 同一个作用域中相同变量名也存在遮蔽现象
  let long_lived_binding = 'a';
  println!("{}", long_lived_binding);
}
