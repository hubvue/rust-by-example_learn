// Drop
// Drop trait 只有一个方法：drop，当对象离开作用域时会自动调用该方法。Drop trait的主要作用是释放实现者的实例拥有的资源。
// Box、Vec、String、File，以及Process是一些实现了Drop trait来释放资源的类型。Drop trait也可以为任何自定义类型手动实现。

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    {
        let _b = Droppable { name: "b" };

        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting Block B");
        }
        println!("Just exited block B");

        println!("Exiting Block A");
    }
    println!("Just exited block A");

    // 变量可以手动调用drop函数来销毁。
    drop(_a);

    println!("end of the main function");

    // _a已经被手动销毁，在这里就不会再次被销毁
}
