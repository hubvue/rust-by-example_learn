// 虚类型参数
// 虚类型参数是一种在运行时不出现，而且仅在编译时进行静态检查的类型参数。

// 可以用额外的泛型类型参数指定数据类型，这类型可以充当标记，也可以供编译时类型检查使用，这个额外的参数没有存储值，也没有运行时行为。

// 下面例子，使用std::marker::PhantomData作为虚类型参数的类型，创建包含不同数据类型的元组。

use std::marker::PhantomData;

// 这个虚元组元组结构体体对A是泛型的，并且带有隐藏参数B
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

// 这个虚类型结构体对A是泛型的，并且带有隐藏参数B
#[derive(PartialEq)] //允许这种类型进行相等测试
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

// 注意：对于泛型A会分配存储空间，但是B不会，因此B不能参与运算。

fn main() {
    // 指定f32和f64是隐藏类型
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhatomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhatomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // println!("{}, {}", _tuple1, _tuple2);
}
