// 引用计数Rc

// 当需要多个所有权时，可以使用Rc（引用计数 Reference Counting）.Rc 跟踪引用的数量，这相当于包裹在Rc值的所有者的数量

// 每当clone一个Rc时，Rc的引用计数就会加1，而每当clone得到的Rc退出作用域时，引用计数就会减1.当Rc的引用计数变为0时，这意味着已经没有所有者，Rc和值都会被删除。

// clone Rc 从不执行深Copy，clone只创建另一个指向包裹值的指针，并增加计数。

use std::rc::Rc;

fn main() {
    let rc_examples = "Rc examples".to_string();
    {
        println!("--- rc_a is created ---");

        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Reference count of rc_a: {}", Rc::strong_count(&rc_a));

        {
            println!("--- rc_a is cloned to rc_b ---");

            let rc_b = Rc::clone(&rc_a);
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            // 如果两者内部的值相等的话，则两个Rc相等
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            // 可以直接使用值的方法
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);

            println!("--- rc_b is dropped out of scope ---");
        }

        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        println!("--- rc_a is dropped out of scope ---");
    }

    // rc_examples已经移动到Rc里面
    // println!("rc_example {}", rc_examples);
}
