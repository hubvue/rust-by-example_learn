// trait 方法中生命周期的标注基本上与函数类似。注意：impl也可能有生命周期的标注。

#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}, {}", b, b.x);
}
