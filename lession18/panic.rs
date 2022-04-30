// panic
// 最简单的错误处理机制是panic，它会打印一个错误，开始回退任务，且通常会退出程序。这里我们显式地在错误条件下调用panic

fn give_princess(gift: &str) {
    if gift == "snake" {
        panic!("AAaaaaaa!!!!");
    }
    println!("I love {}s !!!!!!", gift);
}

fn main() {
    give_princess("teddy bear");
    give_princess("snake");
    println!("after");
}
