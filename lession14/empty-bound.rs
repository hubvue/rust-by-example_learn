// 空约束
// 约束的工作机制会产生这样的效果：即时一个trait不包含任何功能，仍然可用它作为约束。标椎库中的Eq和Ord就是这样的trait。

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// 这些函数只对实现了相应的trait的类型有效。
// 事实上这些trait内部是空的，但这没有关系。
fn red<T: Red>(_: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // println!("A turkey is {}", red(&turkey));
}
