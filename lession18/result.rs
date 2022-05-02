// Result

// Result 是Option类型的更丰富的版本，描述的是可能的错误二不是可能的不存在。
// 也就是说：Result<T, E>可以有两个结果的其中一个
// - Ok<T>：找到T元素
// - Err<E>: 找到E元素，E即表示错误的类型。

// 按照约定，预期结果为Ok，而意外结果为Err

// Result有许多类似Option。例如unwrap()，它要么举出元素T，要么就panic。对于事件的处理，Result和Option有很多相同的组合算子。

// 在使用Rust，可能会遇到返回Result类型的方法，例如parse()方法。它并不是总能把字符串解析成指定的类型，所以parser()返回一个Result表示可能的失败。

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    // 报错
    let tt = multiply("t", "2");
    println!("double is {}", tt);
}

// 在失败的情况下，parse()产生一个错误，留给unwrap()来解包并产生panic。另外，panic会退出我们的程序，并提供一个错误信息。
// 为了改善错误信息的质量，我们应该更具体地了解返回类型并考虑显式地处理错误。
