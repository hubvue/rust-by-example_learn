// 结构体的可见性
// 结构体的字段也是一个可见性的层次。字段默认拥有私有的可见性，也可以加上pub修饰语来重载该行为。
// 只有从结构体被定义的模块之外访问其字段时，这个可见性才会起作用，其意义是隐藏信息（即封装性）。


mod my {
  // 一个公有的结构体，带有一个公有的字段（类型为泛型T）
  pub struct OpenBox<T> {
    pub contents: T
  }
  // 一个公有的结构体，带一个私有的字段（类型为泛型T）
  #[allow(dead_code)]
  pub struct ClosedBox<T> {
    contents: T
  }
  impl<T> ClosedBox<T> {
    // 公有的构造器方法
    pub fn new(contents: T) -> ClosedBox<T> {
      ClosedBox {
        contents: contents
      }
    }
  }
}

fn main() {
  // 带有公有字段的公有结构体，可以像平常一样构造
  let open_box = my::OpenBox{
    contents: "public information"
  };
  // 并且字段可以正常访问到。
  println!("The open box contains: {}", open_box.contents);

  // 带有私有字段的机构提不能使用字段名来构造
  // let closed_box = my::ClosedBox{
  //   aaa: "123"
  // };

  // 不过可以使用公有构造器创建
  let _closed_box = my::ClosedBox::new("classidied information");
  // 私有字段也不可访问
}
