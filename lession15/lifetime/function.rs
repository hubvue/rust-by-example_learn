// 函数
// 排除省略标注的情况，带上生命周期的函数签名有一些限制。
// - 任何引用都必须拥有标注好的生命周期。
// - 任何被返回的引用都必须和某个输入量相同的生命周期或者是静态类型(staitc)

// 另外要注意：如果没有输入的函数返回引用，有时会导致返回的引用指向无效数据，这种情况下进制它返回这样的引用。

// 下面是一些合法的带有生命周期的函数例子。

// 一个拥有生命周期'a的输入引用，其中'a的存活时间至少与函数一样长。
fn print_one<'a>(x: &'a i32) {
    println!("print_one: x is {}", x);
}

// 可变引用同样也可以拥有生命周期
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// 拥有不同生命周期的多个元素。
// 对下面这种情形，两者即时拥有相同的生命周期'a也没问题，但对一些更复杂的场景，可能就需要不同的生命周期了
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("print_multi: x is {}, y is {}", x, y);
}

// 返回传递进来的引用也是可行的。
// 但必须返回正确的生命周期。
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    println!("{:p}", x);
    x
}

// 无效的引用：'a的存活时间必须比函数长。
// 这里&String::from("foo")会创建一个String类型的引用，当数据离开作用域时删除，返回一个指向无效数据的引用。
// fn invalid_output<'a>() -> &'a String {
//     &String::from("foo")
// }

fn main() {
    let x = 7;
    let y = 9;

    print_one(&x);

    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    println!("{:p}, {:p}", &x, z);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
    println!("{}", t);
}
