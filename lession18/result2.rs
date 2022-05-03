// Result 的 map

// result1.rs中的multiply函数的panic设计不是健壮的。一般的，我们希望把错误返回给调用者。这样它就可以决定回应错误的正确方式

// 首先，我们需要了解需要处理的错误类型时什么。为了确定Err的类型，我们用用parse()来试验。
// Rust 已经为i32类型使用FromStr trait实现了parse()。结果表明，这里的Err类型被指定为ParseIntError。

// 怎么确定Err的类型呢？由于目前用于获取类型的函数仍然是不稳定的，因此可以用间接的方法
// fn main() {
//     let i: () = "t".parse::<i32>();
// }
// 上面代码运行报错：Result<i32, ParseIntError>，因此可以发现，Err是ParseIntError类型的。

// 使用match
use std::num::ParseIntError;

// 修改上一节中的返回类型，现在使用模式匹配而不是unwrap()
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

// 幸运的是，Option的map、and_then、以及很多其他组合算子也为Result实现了

fn multiply_fn(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // Ok
    let twenty = multiply("10", "2");
    print(twenty);

    // Err
    let tt = multiply("t", "2");
    print(tt);

    // 组合算子
    let twenty = multiply_fn("10", "2");
    print(twenty);

    let tt = multiply_fn("t", "2");
    print(tt);
}
