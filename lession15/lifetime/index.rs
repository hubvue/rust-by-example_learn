// 生命周期
// 生命周期是这样一种概念：编译器（中的借用检查器）用它来保证所有的借用都是有效的。
// 确切的说，一个变量的声明周期额在它创建的时候开始，在它销毁的时候结束。
// 虽然生命周期和作用域经常被一起提及，但是它们并不相同。

//
// 例如考虑这种情况，我们通过&来借用一个变量。该借用拥有一个生命周期，此声明周期由它声明的位置决定，于是，只要该借用在出借者被销毁前结束，借用就是有效的。
// 然而，借用的作用域则是由使用引用的位置决定的。

fn main() {
    let i = 3; // i的生命周期开始
    {
        let borrow1 = &i; // borrow1 的生命周期开始
        println!("borrow1: {}", borrow1);
    } // borrow1的生命周期结束
    {
        let borrow2 = &i; // borrow2 的生命周期开始
        println!("borrow2: {}", borrow2);
    } // borrow2 的生命周期结束
} // i的生命周期结束
