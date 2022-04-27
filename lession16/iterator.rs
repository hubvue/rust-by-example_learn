// Iterator
// Iterator trait用来对集合类型（比如数组）实现迭代器。
// 这个trait只需要定义一个返回next元素的方法，这可手动在impl代码块中定义，或者自动定义（比如在数组或区间中）

// 为方便起见，for解构会使用.into_iter()方法将一些集合类型转为迭代器。

struct Fibonacci {
    curr: u32,
    next: u32,
}

// 只需要一个next方法
impl Iterator for Fibonacci {
    type Item = u32;
    // 在这里使用 curr和next来定义数列
    // 返回类型为Option<T>
    //  当Iterator结束时，返回None
    //  其他情况，返回Some包裹的下一个值。
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        // 既然斐波那切数列不存在重点，那么Iterator总是返回Some
        Some(self.curr)
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Fibonacci { curr: 1, next: 1 }
    }
}

fn main() {
    // 0..3是一个Iterator，会产生0、1和2。
    let mut sequence = 0..3;

    println!("Four consecutive next calles on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterate through 0..3 using for");
    for i in 0..3 {
        println!("> {}", i);
    }

    // take(n) 方法提取 Iterator 的前 n 项
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in Fibonacci::default().take(4) {
        println!("> {}", i);
    }

    // skip(n) 方法移除前n项，从而缩短了 Iterator
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in Fibonacci::default().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // iter 方法对数组/slice 产生一个 Iterator
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}
