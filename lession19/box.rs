// Box、堆和栈

// 在Rust中国，所有值默认都是栈分配的。通过创建Box<T>，可以把值”装箱“来使值在堆上分配。
// Box是一个智能指针，指向堆分配的T类型的值。当Box离开作用域时，它的析构函数会被调用，内部的对象会被销毁，堆上分配的内存也会被释放。

// 被Box装箱的值可以使用*运算符进行解引用；这会移除掉一层装箱。

use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // 在堆上分配这个点（point），并返回一个指向它的指针
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    // 栈分配的变量
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 },
    };

    // 堆分配的变量
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        p1: origin(),
        p2: origin(),
    });

    // 函数的输出也可以装箱
    let boxed_point: Box<Point> = Box::new(origin());

    // 两层
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("{}", mem::size_of_val(&point));
    println!("{}", mem::size_of_val(&rectangle));

    println!("{}", mem::size_of_val(&boxed_point));
    println!("{}", mem::size_of_val(&boxed_rectangle));

    println!("{}", mem::size_of_val(&box_in_a_box));

    let unboxed_point: Point = *boxed_point;
    println!("{}", mem::size_of_val(&unboxed_point));
}
