// 遍历 Result

// Iter::map 操作可能失败，比如：

// fn main() {
//     let strings = vec!["tofu", "93", "18"];
//     let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
//     println!("Results: {numbers:?}",);
// }

// 以下是处理这种文件的策略。

// 1.使用filter_map() 忽略失败的项
// filter_map 会调用一个函数，过滤掉为None的所有结果。
// Result上的ok方法是将Result转为Option

// fn main() {
//     let strings = vec!["tofu", "93", "18"];
//     let numbers: Vec<_> = strings
//         .into_iter()
//         .filter_map(|s| s.parse::<i32>().ok())
//         .collect();
//     println!("Result: {numbers:?}");
// }

// 2.使用collect()使整个操作失败
// Result实现了FromIter，因此结果的向量（Vec<Result<T, E>>）可以被转换成Result包裹着向量（Result<Vec<T>, E>）,一旦找到一个Result::Err，遍历就被终止

// fn main() {
//     let strings = vec!["tofu", "93", "18"];
//     let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
//     println!("Result: {numbers:?}");
// }

// 3. 使用Partition()收集合法的值与错误
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);

    // 此时numbers和errors依然被Result包裹，因此需要解开
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Number: {numbers:?}");
    println!("Error: {errors:?}");
}
