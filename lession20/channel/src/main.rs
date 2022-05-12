// 通道

// Rust为线程之间的通信提供了异步的通道。通道允许两个端点之间信息的单项流动：Sender（发送端）和Receiver（接收端）

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
fn main() {
    // 通道的两个端点：Sender<T>和Receiver<T>，其中T是要发送的消息的类型
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..3 {
        // sender 端可被复制
        let thread_tx = tx.clone();

        // 每个线程都将通过通道来发送它的id
        thread::spawn(move || {
            // 被创建的线程取得thread_tx的所有权
            // 每个小城都把消息放在通道的消息队列中
            thread_tx.send(id).unwrap();

            // 发送是一个非阻塞操作，线程将在发送消息后会立即继续运行
            println!("thread {} finished", id);
        });
    }

    // 所有消息都在此处被收集
    let mut ids = Vec::with_capacity(3);
    for _ in 0..3 {
        // recv 方法从通道中拿到一个消息
        // 若无可用消息的话， recv将阻止当前线程
        ids.push(rx.recv());
    }
    // 显示消息被发送的次序
    println!("{:?}", ids);
}
