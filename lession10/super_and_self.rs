// 可以在路径中使用super和self关键字，从而在访问项时消除歧义，以防止不必要的路径硬编码

fn function() {
  println!("called function()");
}

mod cool {
  pub fn function() {
    println!("called coll::function()");
  }
}


mod my {
  fn function() {
    println!("called my::function()");
  }
  
  mod cool {
    pub fn function() {
      println!("called my::cool::function()");
    }
  }

  pub fn indirect_call() {
    // 在这里访问所有名称为function的函数
    println!("called my::indirect_call() that \n>");

    // 结果相同
    // self表示当前模块
    self::function();
    function();

    self::cool::function();
    // super表示父作用域
    super::function();
    {
      // crate表示最外层作用域
      use crate::cool::function as root_function;
      root_function();
    }
  }
}

fn main() {
  my::indirect_call();
}
