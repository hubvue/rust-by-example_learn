// 借用

// 多数情况下，我们更希望能访问数据，同时不取得其所有权。为实现这点，Rust使用了借用机制。
// 对象可以通过引用(&T)来传递，从而取代通过值（T）来传递。

// 编译器通过借用检查静态地保证了引用总是指向有效的对象。也就是说，当存在引用指向一个对象时，该对象不能被销毁。

// 此函数取得一个box的所有权并销毁它
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destorying box that contains {}", boxed_i32);
}

// 此函数借用一个i32类型
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    // 创建一个分配在堆上的指针
    let boxed_i32 = Box::new(5_i32);
    // 创建一个分配在栈上的i32类型
    let stacked_i32 = 6_i32;

    // 借用了box的内容，但没有取得所有权，所以box的内容之后可以再次借用。
    // 函数本身就是一个作用域，因此下面两个哈数运行完成之后，在函数中临时创建的引用也就不复存在了。
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // 取得一个对box中数据的引用（借用）
        let _ref_to_i32: &i32 = &boxed_i32;

        // eat_box_i32(boxed_i32);
        // 如果没有下面内容，boxed_i32此时没有任何借用，是可以被销毁的，但是如果后面依然有借用，是不可以销毁的。

        borrow_i32(_ref_to_i32);
        // _ref_to_i32 离开作用域时销毁
    }
    // 此时没有任何变量借用boxed_i32，因此可以直接销毁
    eat_box_i32(boxed_i32);
}

// 借用是指可以访问数据，但是没有数据的所有权，是对值的限制
