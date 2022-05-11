// 线程

// Rust 通过 spawn 函数提供了创建本地操作系统线程的机制，该函数的参数是一个通过值捕获变量的闭包。

// Rust 线程由操作系统调度

use std::thread;

mod map_reduce;

static NTHREADS: i32 = 10;
fn main() {
    // let mut children = vec![];
    // for i in 0..NTHREADS {
    //     children.push(thread::spawn(move || -> i32 {
    //         println!("this is thread number {}", i);
    //         i
    //     }))
    // }

    // for child in children {
    //     // 线程执行结束，返回结果
    //     println!("i: {:?}", child.join());
    // }
    // println!("done");

    map_reduce::map_reduce();
}
