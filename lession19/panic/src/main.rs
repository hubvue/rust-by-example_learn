// panic!

// panic！宏可用于产生一个panic（类似go的panic以及js的 throw error），并开始回退它的函数调用栈。在回退栈的同时，运行时将会释放该线程所拥有的所有资源，这是通过调用现成中所有对象的析构函数完成。

// panic不会发生内存泄漏

// 整型除法的重新实现
fn division(dividend: i32, divisor: i32) -> i32 {
    // 除数为0，触发panic
    if divisor == 0 {
        panic!("division by zero");
    }
    dividend / divisor
}

fn main() {
    // 在堆上分配一个整数
    let _x = Box::new(0i32);

    division(3, 0);

    println!("This point won not be reached!");
}
