// Rust的类型推断不仅仅在初始化时推断值的类型，而且还会考量变量在之后怎么用，借此推断值的类型。

fn main() {
  let ele = 8u8;

  let mut vec = Vec::new();

  vec.push(ele);

  println!("{:?}", vec);
}
