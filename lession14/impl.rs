struct S; // 类型S
struct GenericVal<T>(T); // 泛型类型

// GenericVal 的实现
impl GenericVal<f32> {}

impl GenericVal<S> {}

// 实现的泛型<T> 必须在类型之前写出来
impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// 不带泛型的实现
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// 带泛型的实现
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    println!("{}, {}", x.value(), y.value());
}
