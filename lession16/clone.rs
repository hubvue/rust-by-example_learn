// Clone
// 当处理资源时，默认的行为是在赋值或函数调用的同时将它们转移。但是我们有时候也需要把资源赋值一份。

// Clone trait 正好帮助我们完成这个任务。通常，我们可以使用由Clone trait定义的.clone()方法。

#[derive(Debug, Clone, Copy)]
struct Nil;

#[derive(Debug, Clone)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    let nil = Nil;
    // copy
    let copied_nil = nil;

    println!("{:?}, {:?}", nil, copied_nil);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("{:?}", pair);

    // moved
    let moved_pair = pair;
    println!("copy {:?}", moved_pair);
    // println!("{:?}", pair); moved

    let cloned_pair = moved_pair.clone();
    println!("{:?}, {:?}", moved_pair, cloned_pair);

    drop(moved_pair);
    // println!("moved: {:?}", moved_pair);
    println!("cloned: {:?}", cloned_pair);
}
