// 关联类型
// 通过把容器内部的类型放在trait中作为输出类型，使用关联类型增加了代码的可读性。

// 这样的trait的定义语法如下：

// trait Contains {
//     type A;
//     type B;
//     // 使用类型关联
//     fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
// }

// // 使用关联类型的写法
// fn difference<C: Contains>(container: &C) -> i32 {}

// 使用关联类型写一个之前的例子
struct Container(i32, i32);

trait Contains {
    type A;
    type B;
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;
    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} add {} : {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );

    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
