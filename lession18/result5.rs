// 引入？

// 当使用unwrap去解一个Result类型的时候，如果是Err会产生panic，但是有时候我们需要解Result，但是不希望发生panic
// 因此就可以使用 ?来解，当遇到Err时会直接将Err返回，而不会产生panic

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
