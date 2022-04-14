//  多重约束 可以用+连接，多重约束是且的关系

use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}

// #[derive(Debug)]
// struct Person {
//     name: String,
// }

fn main() {
    // let person = Person { name: "kim" };
    // compare_prints(&person);
    // compare_types(person, person);

    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];
    compare_prints(&string);
    compare_types(&array, &vec);
}
