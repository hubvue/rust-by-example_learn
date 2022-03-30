// TryFrom TryInfo
// 类似于From和Into，TryFrom和TryInto是类型转换的通用trait。
// 不同于From/Into的是，TryFrom和TryInto trait用于易出错的转换，也正因如此，其返回值是Result类型

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
  type Error = ();
  fn try_from(value: i32) -> Result<Self, Self::Error> {
    if value % 2 == 0 {
      Ok(EvenNumber(value))
    } else {
      Err(())
    }
  }
}

fn main() {
  // TryFrom
  assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
  assert_eq!(EvenNumber::try_from(5), Err(()));

  // TryInto
  let result: Result<EvenNumber, ()> = 8i32.try_into();
  assert_eq!(result, Ok(EvenNumber(8)));
  let result: Result<EvenNumber, ()> = 5i32.try_into();
  assert_eq!(result, Err(()));
}

