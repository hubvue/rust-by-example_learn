// Rust不提供原生类型之间的隐式类型转换，但是可以使用as关键字进行显式类型转换。
#![allow(overflowing_literals)]

fn main() {
  let decimal = 65.5321_f32;

  // 报错，不允许隐式类型转换
  // let integer: u8 = decimal;

  // 浮点数转整数是直接去掉小数位
  let integer = decimal as u8;
  let character = integer as char;
  println!("{}", integer);
  println!("{}", character);

  // 当把任何类型转换为无符号类型T时，会不断加上或减去(std::T::MAX + 1)
  // 直到值位于新类型T的范围内

  
  
  // **超过无符号类型T的范围时会报错**
  println!("1000 as a u16 is: {}", 1000 as u16);
  
  // 处理方式：从最低有效位开始保留8位，然后剩余位置，直到最高有效位都被抛弃。
  // MSB是二进制得最高位，LSB是二进制的最低位

  println!("1000 as a u8 is: {}", 100 as u8);

  // 小于无符号类型T的返回会加MAX + 1
  println!(" -1 as a u8 is: {}", (-1i8) as u8);

  println!("{}", 1000 % 256);

  // 当转换到有符号类型时，（位操作的）结果就和“先转换到对应的无符号类型
  // 如果MSB是1，则该值为负“是一样的

  // 当然如果数组已经在目标类型范围内，就直接放进去
  println!("128 as a i16 is: {}", 128 as i16);

  // 128转为u8还是128，但转为i8相当于给128取8位的二进制补码，其值是 -128
  println!("128 as a i8 is: {}", 128 as i8);

  println!("1000 as a u8 is : {}", 1000 as u8);
  println!("2232 as a i8 is : {}", 232 as i8);
}
