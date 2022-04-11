// use声明可以讲一个完整的路径绑定到一个新的名字，从而更容易访问。

use deeply::nested::function as other_function;

fn function() {
  println!("called function()");
}

mod deeply {
  pub mod nested {
    pub fn function() {
      println!("called deeply::nested::function()");
    }
  }
}


fn main() {
  other_function();
  println!("Entering block");
  {
    use deeply::nested::function;
    function();
    println!("Leving block");
  }
  function();
}
