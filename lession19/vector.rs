// 动态数组vector
// vector是大小可变的数组。和slice类似，它们的大小在编译时是未知的，但它们可以随时扩大或缩小。
// 一个vector使用3个词来表示：
//  - 一个指向数据的指针，
//  - vector的长度
//  - vector的容量

// 此容量指明了要为这个vector暴露多少内存。vector的长度只要小于该容量，就可以随意增长；当需要超过这个阈值时，会给vector重新分配一段更大的容量。

fn main() {
    // 迭代器可以被收集到vector中
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("{:?}", collected_iterator);

    // vec! 宏 可用来初始化一个vector
    let mut xs = vec![1i32, 2, 3];
    println!("{:?}", xs);

    // 在vector的尾部插入一个新元素
    xs.push(4);
    println!("xs: {:?}", xs);

    // 不可变的vector是不能被push新元素的
    // collected_iterator.push(0);

    // len 方法用于获取vector的当前大小
    println!("size: {}", xs.len());

    // 可使用索引访问元素
    println!("second element: {}", xs[1]);

    // 使用索引访问超出当前vector的大小会抛出一个panic
    // println!("fourth element: {}", xs[4]);

    // pop方法移除vector的自后一个元素并将它返回, 返回的是一个Option类型
    println!("pop last element {:?}", xs.pop());

    // 使用迭代器遍历
    println!("contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    // 可以在迭代vector的同时，使用独立变量 i 来记录迭代次数
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    // 使用iter_mut，在迭代器过程中修改值
    for x in xs.iter_mut() {
        *x *= 3;
    }

    println!("Updated vector: {:?}", xs);
}
