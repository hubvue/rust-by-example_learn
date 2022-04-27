// impl Trait
// 如果函数范湖实现了MyTrait的类型，可以将其返回类型编写为->impl MyTrait。

use std::iter;
use std::vec::IntoIter;

// 返回迭代器类型的函数
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// 返回 trait的函数
fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// 更重要的是，某些Rust类型无法写出。例如，每个闭包有自己为命名的具体类型。在使用impl Trait语法之前，必须在堆上进行分配才能返回闭包。

// 返回一个将输入和y相加的函数
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| x + y;
    closure
}

// 还可以使用impl trait 返回使用map或filter闭包的迭代器，这使得使用map和filter更容易。
// 因为闭包类型没有名称，所有如果函数返回带闭包的迭代器，则无法写出显式的返回类型。
// 但又了impl Trait就可以实现

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    // let mut v3 = combine_vecs(v1, v2);
    let mut v3 = combine_vecs_explicit_return_type(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");

    let plus_one = make_adder_function(1);
    println!("plus_one: {}", plus_one(2));
    let v5 = vec![1, 2, 3, 4, 5];
    let mut res = double_positives(&v5);
    println!("{:?}", res.next());
    println!("{:?}", res.next());
    println!("{:?}", res.next());
    println!("{:?}", res.next());
    println!("{:?}", res.next());
    println!("{:?}", res.next());
}
