// 别名使用
// 数据可以多次不可变借用，但是在不可变借用的同时，原始数据不能使用可变借用。
// 或者说，同一时间内只允许一次可变借用，仅当最后一次使用可变借用之后，原始数据才可以再次借用。

// 上面说的就是RWLock

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    // 创建两个不可变借用
    let borrowed_point = &point;
    let another_borrow = &point;

    // 报错：在使用不可变借用的同时不可以使用可变借用
    // let mutable_borrow = &mut point;

    // 数据可以通过借用或原始类型来访问
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // 不可变借用使用完销毁，此时可以使用可变借用
    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // 当后面有使用可变借用时，这里会报错，因为当可变借用存在时，不能使用不可变借用
    // let y = &point.y;
    // println!("Point Z coordinate is {}", point.z);

    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    // 当可变借用不再使用时，可以重新进行其他借用
    let new_borrowed_point = &point;
    println!(
        "Point new has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}
