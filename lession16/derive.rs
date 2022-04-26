// 派生
// 通过#[derive]属性，编译器能够提供某些trait的基本实现。如果需要更复杂的行为，这些trait也可以手动实现。
// 下面是可以自动派生的trait
// - 比较trait: Eq, PartialEq, Ord, PartialOrd
// - Clone, 用来从&T创建副本T
// - Copy，使用类型具有Copy语义，而非Move语义
// - Default，创建数据类型的空实例。
// - Debug, 使用{:?}formatter来格式挂一个值

// Centimeters 是一个可以比较的元组结构体
#[derive(PartialEq, PartialOrd, Debug)]
struct Centimeters(f64);

// Inches 是一个可以打印的元组结构体
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

// Seconds 不带附加属性的元组结构体
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);
    // 报错，Seconds没有实现Debug
    // println!("{:?}", _one_second);

    // 报错Seconds没有实现PartialEq
    // let _this_is_true = (_one_second == _one_second);

    let foot = Inches(12);
    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);
    println!("meter {:?}", meter);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };
    println!("One foot is {} than one meter", cmp);
}
