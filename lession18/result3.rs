// 给Result取别名

// 当我们要重用某个Result类型时，该怎么办？回忆一下，Rust允许我们创建别名。若某个Result有可能被重用，我们可以方便地给它取一个别名。

// 在模块的层面上创建别名特别有帮助。同一个模块中的错误常常会有相同的Err类型，所以单个别名就能简便地定义所有相关的Result。以至于标准库也提供了一个别名：io::Result

use std::num::ParseIntError;

// 为带有错误类型ParseIntError的Result定义一个泛型别名。
type AliasedResult<T> = Result<T, ParseIntError>;

// 使用别名
fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
