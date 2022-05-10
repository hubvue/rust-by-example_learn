// 共享引用计数

// 当线程之间所有权需要共享时，可以使用Arc（共享引用计数，Atomic Reference Counted）
// 这个解构通过Clone实现可以为内存堆中的值的位置创建一个引用指针，同时增加引用计数器。
// 由于它在线程之间共享所有权，因此当指向某个值的最后一个引用指针退出作用域时，该变量将被删除。

use std::sync::Arc;
use std::thread;

fn main() {
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            println!("{:?}", apple);
        });
    }
    println!("stop")
}
