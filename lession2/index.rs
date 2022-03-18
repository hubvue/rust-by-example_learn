// 标量类型：
// 有符号整数：i8、i16、i32、i64、i128和isize（指针宽度）
// 无符号整数：u8、u16、u32、u64、u128和usize（指针宽度）
// 浮点数：f32、f64
// char（字符）：单个Unicode字符，如'a'（每个都是4个字节）
// bool（布尔类型）：true、false
// 单元类型：(), 其唯一可能的值就是()这个空元组

// 复合类型：
// 数组：[1,2,3]
// 元组：(1, true)

fn main() {
  // 变量可以给出类型说明
  let logical: bool = true;

  // 常规说明
  let a_float: f64 = 1.0;
  // 后缀说明
  let a_int = 3i64;

  // 整形的默认类型为i32，浮点数的默认类型为f64
  let b_float = 3.14;
  let b_int = 123;

  // 根据上下文推断类型
  let mut c_int = 123;   // 此时c_int为i64类型
  c_int = 4294967296i64;

  // 只有定义可变的变量，其值才可以改变
  let mut d_int = 123;
  d_int = 345;

  // 可以通过遮蔽（也就是变量覆盖）来覆盖前面的变量
  let d_int = 123;
}
