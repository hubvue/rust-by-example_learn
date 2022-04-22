// RAII
// Rust的变量不只是在栈中存储数据；他们也有占有资源，比如Box<T>占有堆中的内存。
// Rust强制实行RAII（资源获取即初始化），所以任何对象在离开作用域时，它的析构函数就被调用，然后它占有的资源就被释放。
// 这种行为避免了资源泄露。

// RAII要求，资源的有效期与持有资源的对象的生命期严格绑定，即由对象的构造函数完成资源的分配（获取），同时由析构函数完成资源的释放。在这种要求下，只要对象能正确地析构，就不会出现资源泄露问题。

// fn create_box() {
//     // 在堆上分配一个整型数据
//     let _box1 = Box::new(3i32);
//     // 作用域结束，变量在这里被销毁，内存得到释放。
// }
// fn main() {
//     // 在堆上分配一个整型数据；
//     let _box2 = Box::new(5i32);

//     // 嵌套作用域
//     {
//         // 在堆上分配一个整型数据
//         let _box3 = Box::new(4i32);
//         // 作用域结束，_box3被销毁，内存释放
//     }
//     // 创建一堆box
//     for _ in 0u32..1_000 {
//         create_box();
//     }

//     // 作用域结束，_box2被销毁，内存释放
// }

// 可以使用 valgrind 对内存错误进行仔细检查：
// rustc raii.rs && valgrind ./raii
// note：我的电脑下载太麻烦就懒得搞了

// 析构函数
// Rust中的析构函数概念是通过Drop trait提供的。当资源离开作用域时，就调用析构函数。

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let _x = ToDrop;
    println!("Made a ToDrop!");
}
