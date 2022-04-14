// 约束
// 在使用泛型时，类型参数常常使用trait作为约束来明确规定类型应实现哪些功能。

// 例如下面例子使用Display来约束T，也就是说T必须实现Display

// fn printer<T: Display>(t: T) {
//     println!("{}", t);
// }

// 约束会报泛型类型限制为符合约束的类型。
// 约束的另一个作用是泛型的实例可以访问作为约束的trait的方法

use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };
    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));
}
