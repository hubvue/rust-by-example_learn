// 把错误“装箱”

// 如果又想写简单的代码，又想保存原始错误信息，一个方法是把它们装箱（Box）。这做得坏处就是被包装的错误类型只能在运行时了解，而不能被静态地判别。

// 对任何实现了Error trait的类型，标准库的Box听过From 为它们提供了到Box<Error>的类型转换。

use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {
    fn description(&self) -> &str {
        "invalid first item to double"
    }
    fn cause(&self) -> Option<&dyn error::Error> {
        // 泛型错误。没有记录其内部原因
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first().ok_or_else(|| EmptyVec.into()).and_then(|s| {
        println!("first: {:?}", vec);
        s.parse::<i32>().map_err(|e| e.into()).map(|i| {
            println!("{:?}", vec);
            2 * i
        })
    })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
