// where 子句
// 约束也可以使用where子句来表达，它放在{的前面，而不需写在类型第一次出现之前。
// 另外 where 从句可以用于任意类型的限定，而不局限于类型参数本身。

// 应用场景1：分别指定泛型的类型和约束会更清晰

// trait MyTrait<A, D> {
//     fn trait_fn(_a: &A, _d: &D) {}
// }

// struct MyStruct {}

// // no where
// impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for MyStruct {}

// // where
// impl<A, D> MyTrait<A, D> for MyStruct
// where
//     A: TraitB + TraitC,
//     D: TraitE + TraitF,
// {
// }

// 应用场景2：使用where从句比正常语法更有表现力。
use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}
