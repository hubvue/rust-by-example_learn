// 元组是一个可以包含各种类型值的集合，通过()来构建，而每个元组自身又是一个类型标记为 (T1, T2, ...) 的值，其中 T1、T2 是每个元素的类型。

// 元组可以充当函数的参数和返回值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
  let (interger, boolean) = pair;
  (boolean, interger)
}

fn main() {
  let (boolean, interger) = reverse((123, true));
  println!("{},{}", boolean, interger);

  // 包含各种不同类型的元组
  
}
