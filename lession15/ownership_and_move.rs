// 所有权和移动
// 因为变量要负责释放它们所拥有的的资源，所以资源只能有一个所有者。这也防止了资源的重复释放。也并非所有变量都拥有资源（引用）

// 在进行赋值、传递函数参数、函数返回值时，资源的所有权会发生转移。Rust的说法：资源的移动。

// 在移动资源之后，原本的所有者不能再被使用，这可避免悬挂指针的产生。

// 此函数取得对分配的内存的所有权
fn destory_box(c: Box<i32>) {
    println!("Destorying a box that contains {}", c);
    // 作用域结束， c变量被销毁，资源释放
}

fn main() {
    // 栈分配的整型
    let x = 5u32;

    // 将 x 复制到y，不存在资源移动
    let y = x; // Copy

    // 两个值各自都可以使用
    println!("x is {}, and y is {}", x, y);

    // a 死一个指向堆分配的整数的指针
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // 把 a 移动到 b
    let b = a; // Move 把a中存储的指向堆的指针转移到b上，现在是b拥有它，而a不再拥有

    // 此时再访问变量a，则报错 borrow of moved value: `a`
    // println!("a contrains: {}", a);

    // 变量b作为函数参数，b所拥有的指针发生移动
    destory_box(b);

    // 此时在访问b，则报错 borrow of moved value: `b`
    // println!("b contains: {}", b);

    // 可变性
    // 当所有权移动式，可以更改数据的可变性，也可以通过变量遮蔽的方法，使当前变量所拥有的数据的可变性发生改变
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // error: 变量被定义为不可变的
    // *immutable_box = 10;

    // 所有权移动，可变性改变
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    *mutable_box = 10;

    println!("mutable_box contains {}", mutable_box);

    // 部分移动
    // 在单个变量的解构内，可以同时使用by-move和by-reference模式绑定。这样做将导致变量的部分移动，
    // 这意味着变量的某些部分将被移动，二其他部分将保留。在这种情况下，后面不能整体使用父级变量，但是仍然可以使用只引用而不移动的部分。
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    // name 从 person中移出， 但 age只是借用
    let Person { name, ref age } = person;
    println!("name is {}", name);
    println!("age is {}", age);

    //因为部分值的移动，无法访问整体
    // println!("person is {:?}", person);

    // 但是可以访问未移动的部分
    println!("person age is {}", age);
}

// 总结
// 变量所有权最基本的两种语义：Copy语义和Move语义
// 默认为Move语义：当变量在发生赋值、函数传参、函数返回值时，若实现了默认会发生所有权移动。
// 只有当变量实现了Copy语义时，才会进行Copy（浅Copy）。

// 默认实现的Copy语义类型：https://doc.rust-lang.org/std/marker/trait.Copy.html#implementors

// 可变性
// 值并不存在可不可变的概念，值只能被变量所拥有。
// 可变不变是对变量的修饰，也就是变量操作值的权限
