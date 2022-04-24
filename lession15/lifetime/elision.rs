// 省略
// 有些生命周期的模式太常用了，所以借用检查器将会隐式地添加它们以减少程序输入量和增强可读性。
// 这种隐式添加生命周期的过程成为省略。

// 下面两种方法事实上拥有相同的签名
// elided_input的生命周期标注会被编译器自动添加
fn elided_input(x: &i32) {
    println!("elided_input: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("annotated_input: {}", x);
}

// 下面两种函数也拥有相同的签名
fn elided_pass(x: &i32) -> &i32 {
    x
}

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}

fn main() {
    let x = 3;
    elided_input(&x);
    annotated_input(&x);

    println!("{}", elided_pass(&x));
    println!("{}", annotated_pass(&x));
}
