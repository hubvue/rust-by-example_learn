// 强制转换

// 一个较长的生命周期可以强制转为一个较短的生命周期，可以使其在一个通常情况下不能工作的作用域内也能正常工作。
// 强制转换可由编译器隐式地推导并执行，也可以通过声明不同的生命周期的形式实现。

fn mutiply_simple(first: &i32, second: &i32) -> i32 {
    first * second
}

// Rust推导了一个尽可能短的生命周期，然后这两个引用被强制转为这个生命周期，也就是两个引用生命周期的交集。
fn mutiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// <'a: 'b, 'b>读作生命周期 'a 至少要和 'b一样长
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // 较长的生命周期
    {
        let second = 3; //较短的生命周期
        println!("the is {}", mutiply_simple(&first, &second));
        println!("the product is {}", mutiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }
}
