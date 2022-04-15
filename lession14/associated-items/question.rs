// 存在问题：
// trait如果对实现它的容器类型时泛型的，则须遵守类型规范要求--trait的使用者必须指出trait的全部泛型类型。

struct Container(i32, i32);

// 这个trait 检查给定的2个项是否存储在容器中
// 并且能够获取容器的第一个或第二个值。
trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool; //显示要求A 和 B
    fn first(&self) -> i32; //未显示要求A 或 B
    fn last(&self) -> i32; // 未显示要求 A 或 B
}

impl Contains<i32, i32> for Container {
    // 如果存储的数字和给定的相等则微针
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // 得到第一个数字
    fn first(&self) -> i32 {
        self.0
    }
    // 得到最后一个数字
    fn last(&self) -> i32 {
        self.1
    }
}

// 容器C就包含了A和B类型。鉴于C，必须指出A和B显得很麻烦。
fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );

    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
