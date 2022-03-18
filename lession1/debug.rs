// 所有的类型，若想用std::fmt的格式化打印，都要求实现至少一个可打印的traits。自动的实现只为一些类型提供，比如std库中国的类型。所有的其他类型都必须手动实现。

// fmt::Debug这个trait使这项同坐变得简单，所有的类型都能推导fmt::Debug的实现。但是fmt::Display需要手动实现

// 这个结构体不能使用 Display和 Debug来打印
// struct UnPrintable(i32);

// `derive` 属性会自动创建所需的实现，使这个struct可以使用Debug来打印
// #[derive(Debug)]
// struct DebugPrintable(i32);


#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
  // let a = UnPrintable(12);
  // println!("{:?}", a);  // `UnPrintable` cannot be formatted using `{:?}`

  // let b = DebugPrintable(123);
  // println!("{:?}", b);


  println!("Now {:?} will print!", Structure(3));
  println!("Now {:#?} will print!", Deep(Structure(7)));
  // 只打印7
  println!("Now {:#?} will print!", Deep(Structure(7)).0.0);

}
