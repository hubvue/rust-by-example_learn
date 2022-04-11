// 创建一个库，然后看看如果把它链接到另一个crate。

pub fn public_function() {
  println!("called rarys public_function()");
}

fn private_function() {
  println!("called rarys private_function()");
}

pub fn indirect_access() {
  print!("called rarys indirect_access(), that \n >");
  private_function();
}

// 使用 rustc 编译库
// rust --crate-type=lib lib.rs

// 默认情况下，库会使用crate文件的名字，前面加上lib前缀，但这个默认名称可以使用crate_name属性覆盖。
