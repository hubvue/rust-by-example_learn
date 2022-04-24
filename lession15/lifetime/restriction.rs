// 生命周期约束
// 和泛型一样，生命周期也可以使用约束。:字符的意思稍微有些不同。
// 1. T: 'a 在T中所有引用的生命周期都必须比'a长；
// 2. T: Trait + 'a T类型必须实现`Trait` trait，并且在T中所有引用的生命周期都必须比'a长；

use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print<T>(t: T)
where
    T: Debug,
{
    println!("print: t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("print_ref: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
    print(ref_x);
}
