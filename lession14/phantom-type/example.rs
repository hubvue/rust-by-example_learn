// 通过实现一个带虚类型参数的Add trait 可以实现单位检查。

use std::marker::PhantomData;
use std::ops::Add;

/// 创建空枚举类型来表示单位。
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

// Length是一个带有虚类型参数Unit的类型
// 而且对于表示长度的类型f64而言，Length不是泛型。
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

// Add trait 定义了 + 运算符的行为。
impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    // add 返回一个含有和的西你的Length结构体
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // + 调用了针对 f64类型的Add实现。
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(10000.0, PhantomData);

    // + 调用了我们对Length<Unit>实现的 add 方法
    // 由于Length实现了 Copy add 不会消耗 one_foot 和 one_meter，而是复制他们作为self和rhs
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one_foot + one_foot = {:?} in", two_feet.0);
    println!("one_meter + one_meter = {:?} mm", two_meters.0);

    // 类型不匹配的计算会失败
    // let one_feter = one_foot + one_meter;
}
