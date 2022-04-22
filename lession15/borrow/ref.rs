// ref 模式

// 在通过let绑定来进行模式匹配或解构时，ref关键字可用来创建结构体/元组的字段的引用。

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let c = 'Q';
    println!("{}", c);

    // 两种不同的借用方式，效果是一样
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("{:p}, {:p}, {:p}", &c, ref_c1, ref_c2);

    let point = Point { x: 0, y: 0 };

    // 在解构一个结构体时 ref同样有效。
    let _copy_of_x = {
        let Point {
            x: ref ref_to_x, ..
        } = point;

        *ref_to_x
    };
    println!("{}", _copy_of_x);

    // point的可变copy
    let mut mutable_point = point;
    {
        // ref可以与mut结合以创建可变引用
        let Point {
            y: ref mut mut_ref_to_y,
            ..
        } = mutable_point;
        *mut_ref_to_y = 1;
    }

    println!("Point is ({}, {})", point.x, point.y);
    println!(
        "mutable_point is ({}, {})",
        mutable_point.x, mutable_point.y
    );

    // 包含一个指针的可变元组
    let mut mutable_tuple = (Box::new(5u32, 3u32));
}
