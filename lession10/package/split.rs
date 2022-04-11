// 此声明将会查找名为my.rs或my/mod.rs的文件，并将该文件的内容放到此作用域中一个名为my的模块里
mod my;

fn function() {
  println!("called function()");
}

fn main() {
  my::function();
  function();

  my::indirect_access();
  my::nested::function();
}
