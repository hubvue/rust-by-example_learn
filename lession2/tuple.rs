use std::fmt;
// 元组是一个可以包含各种类型值的集合，通过()来构建，而每个元组自身又是一个类型标记为 (T1, T2, ...) 的值，其中 T1、T2 是每个元素的类型。

// 元组可以充当函数的参数和返回值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
  let (interger, boolean) = pair;
  (boolean, interger)
}


#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
  }
}

fn transpose(matrix: Matrix) -> Matrix {
  Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {
  let (boolean, interger) = reverse((123, true));
  println!("{},{}", boolean, interger);

  // 包含各种不同类型的元组
  let long_tuple = (1u8, 2u16, 3u32, 4u64, 5u128,  -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);
  // 通过元组下标访问元素
  println!("{}", long_tuple.1);
  // 元组也支持嵌套，一个元组作为另一个元组的元素
  let tuple_of_tuple = ((1u8, 2u16), (-1i8), (0.1f32));
  println!("{:?}", tuple_of_tuple);

  // 当元组很长的时候，无法输出元组
  // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
  // println!("too long tuple: {:?}", too_long_tuple);

  // 当创建单元素元组时，需要在元素后面多添加一个逗号，这是为了和被括号包含的元素字面量区分。
  let single_tuple = (1u8,);
  println!("{:?}", single_tuple);

  // 元组也可以被解构，从而将值绑定给变量
  let tuple = (1, 0.1, true, "Hello world");
  let (a, b, c, d) = tuple;
  println!("{}, {}, {}, {}, {:?}", a, b, c, d, tuple);

  let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("{:?}", matrix);
  println!("{}", matrix);
  println!("{}", transpose(matrix));
} 
