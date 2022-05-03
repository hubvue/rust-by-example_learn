// 定义一个错误类型

// 有时候把所有不同的错误都视为一种错误类型会简化代码。
// Rust允许我们定义自己的错误类型，一般来说，一个“好的”错误类型应当：
//  - 用同一种类型代表了多种错误
//  - 向用户提供了清楚的错误信息
//  - 能够容易地与其他类型比较
//  - 能够容纳错误的具体信息
//  - 能够与其他错误很好的整合

use std::error;
use std::fmt;

#[derive(Debug, Clone)]
struct DoubleError;

type Result<T> = std::result::Result<T, DoubleError>;

// 实现Display，用于输出
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to doubele")
    }
}

// 实现Error trait
impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // 泛型错误，没有记录其内部原因。
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|s| s.parse::<i32>().map_err(|_| DoubleError).map(|i| 2 * i))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "19"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
