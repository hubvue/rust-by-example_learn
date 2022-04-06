// 默认情况下，模块中的项拥有私有的可见性，不过可以加上pub修饰语来冲在这一行为。
// 模块中只有公有的public的项可以从模块外的作用域访问。

mod my_mod {
  // 模块中的项默认具有私有的可见性。
  fn private_function() {
    println!("called my_mod::private_function()");
  }
  // 使用 pub 修饰语来改变默认可见性。
  pub fn function() {
    println!("called my_mod::function()");
  }
  pub fn indirect_accss() {
    print!("called my_mod::indirect_access(), that \n>");
    private_function();
  }

  // 模块也可以嵌套
  pub mod nested {
    pub fn function() {
      println!("called my_mod::nested::function()");
    }
    #[allow(dead_code)]
    fn private_function() {
      println!("called my_mod::nested::private_function()");
    }
    // 使用 pub(in path) 语法定义的函数只在给定的路径中可见。
    // path必须是父模块，或者是祖先模块。
    pub(in crate::my_mod) fn public_function_in_my_mod() {
      print!("called my_mod::nested::public_function_in_my_mod(), that\n >");
      public_function_in_nested();
    }
    // 使用 pub(self) 语法定义的函数则只在当前模块中可见。
    pub(self) fn public_function_in_nested() {
      println!("called my_mod::nested::public_function_in_nested");
    }
    // 
    pub(super) fn public_function_in_super_mod() {
      println!("called my_mod::nested::public_function_in_super_mod");
    }
  }
  
  pub fn call_public_function_in_my_mod() {
    print!("called my_mod::call_public_function_in_super_mod \n >");
    nested::public_function_in_my_mod();
    print!(">");
    nested::public_function_in_super_mod();
  }

  pub(crate) fn public_function_in_crate() {
    println!("called my_mod::public_function_in_crate()");
  }
  mod private_nested {
    #[allow(dead_code)]
    pub fn function() {
      println!("called my_mod::private_nested::function()");
    }
  }
}


fn function() {
  println!("called function()");
}

fn main() {
  // 模块机制消除了相同名字的项之间的歧义
  function();
  my_mod::function();

  // 公有项，包括嵌套在模块内的，都可以在父模块外部访问。
  my_mod::indirect_accss();
  my_mod::nested::function();
  my_mod::call_public_function_in_my_mod();

  // pub(crate)项可以在同一个crate中的任何地方访问
  my_mod::public_function_in_crate();

  // pub(in path) 项只能在指定的模块中访问
  // my_mod::nested::public_function_in_my_mod();// error is private

  // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的
  // my_mod::private_function();  error
}
