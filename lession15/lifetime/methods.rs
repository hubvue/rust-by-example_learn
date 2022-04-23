// 方法的标注和函数类似

// 方法一般是不需要标明生命周期的，因为self的生命周期会赋给所有的输出生命周期参数。

struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }

    fn print<'a>(&'a self) {
        println!("print: {}", self.0);
    }
}

fn main() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}
