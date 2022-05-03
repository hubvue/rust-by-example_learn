// 从Option中取出Result

// 处理混合错误类型的最基本的手段就是让它们相互包含。

use std::num::ParseIntError;

// fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
//     vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
// }

// 有时候我们不想再处理错误(比如使用?的时候)，但如果Option是None则继续处理错误。一些组合算子可以让我们轻松的交换Result和Option
fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));
    println!("opt, {:?}", opt);
    opt.map_or(Ok(None), |r| r.map(Some)) //解析成了Result类型
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let string = vec!["tofu", "93", "18"];

    println!("{:?}", double_first(numbers));
    println!("{:?}", double_first(empty));
    println!("{:?}", double_first(string));
}
